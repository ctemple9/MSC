//! `GET /v1/performance` — latest bounded performance snapshot for the
//! active server.

use axum::Json;
use axum::extract::State;
use msc_api::dto::{PerformanceMetricNumberDto, PerformanceSnapshotDto};

use crate::routes::lifecycle::LifecycleRoutesState;

pub async fn performance(
    State(state): State<LifecycleRoutesState>,
) -> Json<PerformanceSnapshotDto> {
    let snapshot = state.performance_snapshot();
    let world_time = state.live_world_time();
    Json(PerformanceSnapshotDto {
        ts: snapshot.ts,
        tps_1m: metric(snapshot.tps_1m, "performance.tps"),
        tps_5m: metric(snapshot.tps_5m, "performance.tps"),
        tps_15m: metric(snapshot.tps_15m, "performance.tps"),
        players_online: snapshot.players_online,
        cpu_percent: metric(snapshot.cpu_percent, "performance.cpu"),
        ram_used_mb: metric(snapshot.ram_used_mb, "performance.ram"),
        ram_max_mb: metric(snapshot.ram_max_mb, "performance.ram"),
        world_size_mb: metric(snapshot.world_size_mb, "performance.world-size"),
        world_day: world_time.map(|time| time.day),
        world_time_ticks: world_time.map(|time| time.daytime_ticks),
        server_type: snapshot.server_type,
        runtime: state
            .active_config_server()
            .filter(|server| server.server_type == msc_domain::identity::ServerType::Bedrock)
            .map(|_| state.bedrock_runtime_state()),
    })
}

fn metric(value: Option<f64>, help_id: &'static str) -> Option<PerformanceMetricNumberDto> {
    value.map(|value| PerformanceMetricNumberDto {
        value,
        help_id: Some(help_id.to_string()),
    })
}

/// Read host resources without requiring a selected or running server.
pub async fn host_memory() -> Json<msc_api::dto::HostMemoryDto> {
    let memory = tokio::task::spawn_blocking(msc_infrastructure::metrics::host_memory_bytes)
        .await
        .ok()
        .flatten();
    Json(msc_api::dto::HostMemoryDto {
        used_bytes: memory.map(|(used, _)| used),
        total_bytes: memory.map(|(_, total)| total),
    })
}
