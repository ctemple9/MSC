use clap::{Args, Subcommand, ValueEnum};
use msc_infrastructure::service::{
    ServiceInstallRequest, ServiceManager, ServiceManagerCommand, ServiceName, ServiceState,
    ServiceStatusReport,
};
use serde::Serialize;

use super::{CliError, CommonArgs};

const ENGINE_SERVICE_NAME: &str = "com.ctemple.msc2.agent";

pub(crate) struct LocalEngineStatus {
    pub report: ServiceStatusReport,
    pub boot_enabled: Option<bool>,
}

pub(crate) fn local_engine_status() -> Result<LocalEngineStatus, CliError> {
    let report = service_manager()?
        .execute(ServiceManagerCommand::Status {
            service_name: ServiceName::new(ENGINE_SERVICE_NAME),
        })
        .map_err(service_error)?;
    let boot_enabled = if report.state == ServiceState::NotInstalled {
        None
    } else {
        Some(local_engine_boot_enabled().map_err(service_error)?)
    };
    Ok(LocalEngineStatus {
        report,
        boot_enabled,
    })
}

pub(crate) fn change_local_engine(action: EngineAction) -> Result<(), CliError> {
    #[cfg(not(target_os = "macos"))]
    let manager = service_manager()?;
    #[cfg(not(target_os = "macos"))]
    let service_name = ServiceName::new(ENGINE_SERVICE_NAME);
    let current = local_engine_status()?;
    if current.report.state == ServiceState::NotInstalled {
        return Err(CliError::usage(
            "the local MSC background service is not installed",
        ));
    }
    match action {
        EngineAction::Enable | EngineAction::Disable => {
            set_local_engine_boot_enabled(matches!(action, EngineAction::Enable))
                .map_err(service_error)?;
        }
        EngineAction::Start if current.report.state == ServiceState::Running => {}
        EngineAction::Stop if current.report.state == ServiceState::Stopped => {}
        #[cfg(target_os = "macos")]
        EngineAction::Start => {
            msc_platform_macos::service::start_local_engine().map_err(service_error)?;
        }
        #[cfg(target_os = "macos")]
        EngineAction::Stop => {
            msc_platform_macos::service::stop_local_engine().map_err(service_error)?;
        }
        #[cfg(not(target_os = "macos"))]
        EngineAction::Start => {
            manager
                .execute(ServiceManagerCommand::Start { service_name })
                .map_err(service_error)?;
        }
        #[cfg(not(target_os = "macos"))]
        EngineAction::Stop => {
            manager
                .execute(ServiceManagerCommand::Stop { service_name })
                .map_err(service_error)?;
        }
        EngineAction::Status => {}
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum EngineTarget {
    #[value(alias = "agent", alias = "service")]
    Engine,
}

#[derive(Debug, Clone, Copy)]
pub enum EngineAction {
    Start,
    Stop,
    Status,
    Enable,
    Disable,
}

pub fn run_engine(
    common: CommonArgs,
    _target: EngineTarget,
    action: EngineAction,
) -> Result<(), CliError> {
    if !matches!(action, EngineAction::Status) {
        change_local_engine(action)?;
    }
    let LocalEngineStatus {
        report,
        boot_enabled,
    } = local_engine_status()?;
    let action_name = match action {
        EngineAction::Start => "start",
        EngineAction::Stop => "stop",
        EngineAction::Status => "status",
        EngineAction::Enable => "enable",
        EngineAction::Disable => "disable",
    };
    if common.json {
        let output = EngineServiceOutput {
            command: action_name,
            installed: report.state != ServiceState::NotInstalled,
            state: state_name(report.state),
            boot_enabled,
            pid: report.pid,
        };
        println!(
            "{}",
            serde_json::to_string(&output).map_err(|error| {
                CliError::internal(format!("failed to encode JSON output: {error}"))
            })?
        );
    } else {
        let state = match report.state {
            ServiceState::NotInstalled => "not installed",
            ServiceState::Stopped => "installed, stopped",
            ServiceState::Running => "installed, running",
        };
        let boot = match boot_enabled {
            Some(true) => ", starts at boot",
            Some(false) => ", disabled at boot",
            None => "",
        };
        println!("background service: {state}{boot}");
    }
    Ok(())
}

fn service_error(error: msc_infrastructure::service::ServiceError) -> CliError {
    CliError::internal(format!("local background service: {error}"))
}

fn state_name(state: ServiceState) -> &'static str {
    match state {
        ServiceState::NotInstalled => "notInstalled",
        ServiceState::Stopped => "stopped",
        ServiceState::Running => "running",
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EngineServiceOutput {
    command: &'static str,
    installed: bool,
    state: &'static str,
    boot_enabled: Option<bool>,
    pid: Option<u32>,
}

fn local_engine_boot_enabled() -> Result<bool, msc_infrastructure::service::ServiceError> {
    #[cfg(target_os = "linux")]
    {
        return msc_platform_linux::service::local_engine_boot_enabled();
    }
    #[cfg(target_os = "macos")]
    {
        return msc_platform_macos::service::local_engine_boot_enabled();
    }
    #[cfg(target_os = "windows")]
    {
        return msc_platform_windows::service::local_engine_boot_enabled();
    }
    #[allow(unreachable_code)]
    Err(msc_infrastructure::service::ServiceError::Unsupported(
        "unsupported platform".into(),
    ))
}

fn set_local_engine_boot_enabled(
    enabled: bool,
) -> Result<(), msc_infrastructure::service::ServiceError> {
    #[cfg(target_os = "linux")]
    {
        return msc_platform_linux::service::set_local_engine_boot_enabled(enabled);
    }
    #[cfg(target_os = "macos")]
    {
        return msc_platform_macos::service::set_local_engine_boot_enabled(enabled);
    }
    #[cfg(target_os = "windows")]
    {
        return msc_platform_windows::service::set_local_engine_boot_enabled(enabled);
    }
    #[allow(unreachable_code)]
    Err(msc_infrastructure::service::ServiceError::Unsupported(
        "unsupported platform".into(),
    ))
}

#[derive(Debug, Clone, Subcommand)]
pub enum ServiceCommand {
    /// Register the engine as a background service.
    Install(ServiceInstallArgs),
    /// Remove the background service registration.
    Uninstall(ServiceTargetArgs),
    /// Start the installed background service.
    Start(ServiceTargetArgs),
    /// Stop the installed background service.
    Stop(ServiceTargetArgs),
    /// Show whether the background service is installed or running.
    Status(ServiceTargetArgs),
}

#[derive(Debug, Clone, Args)]
pub struct ServiceInstallArgs {
    /// Stable local service name, for example `msc-engine`.
    #[arg(long)]
    pub service_name: String,

    /// Path to the `msc` binary the service should launch.
    #[arg(long)]
    pub binary_path: String,

    /// Directory the service should treat as its current working directory.
    #[arg(long)]
    pub working_directory: String,

    /// Where the platform adapter should send the service's own stdout/stderr.
    #[arg(long)]
    pub log_path: String,

    /// User account the service should run as, if the platform supports it.
    #[arg(long)]
    pub run_user: Option<String>,

    /// TCP port the service should expose the loopback management API on.
    #[arg(long)]
    pub expected_port: u16,

    /// Extra argument passed to the engine binary. Repeat for multiple values.
    #[arg(long = "arg")]
    pub arguments: Vec<String>,

    /// Extra environment entry in `KEY=VALUE` form. Repeat for multiple values.
    #[arg(long = "env", value_name = "KEY=VALUE")]
    pub environment: Vec<String>,
}

#[derive(Debug, Clone, Args)]
pub struct ServiceTargetArgs {
    /// Stable local service name, for example `msc-engine`.
    #[arg(long)]
    pub service_name: String,
}

pub async fn run(common: CommonArgs, command: ServiceCommand) -> Result<(), CliError> {
    let manager = service_manager()?;
    run_with_manager(common, command, manager.as_ref()).await
}

pub(crate) async fn run_with_manager(
    common: CommonArgs,
    command: ServiceCommand,
    manager: &dyn ServiceManager,
) -> Result<(), CliError> {
    let model = into_model(command)?;
    let command = describe_command(&model);
    let report = manager
        .execute(model)
        .map_err(|error| CliError::internal(format!("service management failed: {error}")))?;

    if common.json {
        let output = ServiceCommandOutput::from_report(command, &report);
        let rendering = serde_json::to_string(&output)
            .map_err(|err| CliError::internal(format!("failed to encode JSON output: {err}")))?;
        println!("{rendering}");
    } else {
        println!("{command}: {}", describe_report(&report));
    }

    Ok(())
}

fn service_manager() -> Result<Box<dyn ServiceManager>, CliError> {
    #[cfg(target_os = "macos")]
    {
        return Ok(Box::new(
            msc_platform_macos::service::MacosLaunchdServiceManager::new(),
        ));
    }
    #[cfg(target_os = "windows")]
    {
        return Ok(Box::new(
            msc_platform_windows::service::WindowsServiceManager::new(),
        ));
    }
    #[cfg(target_os = "linux")]
    {
        return Ok(Box::new(
            msc_platform_linux::service::LinuxSystemdServiceManager::new(),
        ));
    }
    #[allow(unreachable_code)]
    Err(CliError::internal(
        "this platform has no MSC service manager",
    ))
}

/// Returns the data directory configured for the installed local engine.
/// Unix CLI authorization sockets live beside that engine's state, which may
/// be outside the CLI process's default directory.
#[cfg(any(target_os = "macos", target_os = "linux"))]
pub(crate) fn installed_engine_data_dir() -> Result<Option<std::path::PathBuf>, CliError> {
    let report = service_manager()?
        .execute(ServiceManagerCommand::Status {
            service_name: ServiceName::new(ENGINE_SERVICE_NAME),
        })
        .map_err(|error| CliError::internal(format!("service management failed: {error}")))?;
    Ok(report
        .definition
        .and_then(|definition| definition.environment.get("MSC2_DATA_DIR").cloned())
        .map(std::path::PathBuf::from))
}

fn into_model(command: ServiceCommand) -> Result<ServiceManagerCommand, CliError> {
    match command {
        ServiceCommand::Install(args) => Ok(ServiceManagerCommand::Install(args.into_request()?)),
        ServiceCommand::Uninstall(args) => Ok(ServiceManagerCommand::Uninstall {
            service_name: ServiceName::new(args.service_name),
        }),
        ServiceCommand::Start(args) => Ok(ServiceManagerCommand::Start {
            service_name: ServiceName::new(args.service_name),
        }),
        ServiceCommand::Stop(args) => Ok(ServiceManagerCommand::Stop {
            service_name: ServiceName::new(args.service_name),
        }),
        ServiceCommand::Status(args) => Ok(ServiceManagerCommand::Status {
            service_name: ServiceName::new(args.service_name),
        }),
    }
}

impl ServiceInstallArgs {
    fn into_request(self) -> Result<ServiceInstallRequest, CliError> {
        let mut request = ServiceInstallRequest::new(
            self.service_name,
            self.binary_path,
            self.working_directory,
            self.log_path,
            self.expected_port,
        )
        .args(self.arguments);
        if let Some(run_user) = self.run_user {
            request = request.run_user(run_user);
        }
        for entry in self.environment {
            let (key, value) = parse_env_entry(&entry)?;
            request = request.env(key, value);
        }
        Ok(request)
    }
}

fn parse_env_entry(entry: &str) -> Result<(String, String), CliError> {
    let (key, value) = entry.split_once('=').ok_or_else(|| {
        CliError::usage(format!(
            "invalid --env {entry:?}; expected KEY=VALUE so the service model stays explicit"
        ))
    })?;
    if key.trim().is_empty() {
        return Err(CliError::usage(format!(
            "invalid --env {entry:?}; the key cannot be empty"
        )));
    }
    Ok((key.to_string(), value.to_string()))
}

fn describe_command(command: &ServiceManagerCommand) -> String {
    match command {
        ServiceManagerCommand::Install(request) => format!(
            "install {} -> {} (cwd {}, port {})",
            request.service_name.as_str(),
            request.binary_path.display(),
            request.working_directory.display(),
            request.expected_port
        ),
        ServiceManagerCommand::Uninstall { service_name } => {
            format!("uninstall {}", service_name.as_str())
        }
        ServiceManagerCommand::Start { service_name } => format!("start {}", service_name.as_str()),
        ServiceManagerCommand::Stop { service_name } => format!("stop {}", service_name.as_str()),
        ServiceManagerCommand::Status { service_name } => {
            format!("status {}", service_name.as_str())
        }
    }
}

fn describe_report(report: &ServiceStatusReport) -> String {
    match report.state {
        ServiceState::NotInstalled => format!("{} is not installed", report.service_name.as_str()),
        ServiceState::Stopped => {
            format!("{} is installed and stopped", report.service_name.as_str())
        }
        ServiceState::Running => format!(
            "{} is running{}",
            report.service_name.as_str(),
            report
                .pid
                .map(|pid| format!(" (pid {pid})"))
                .unwrap_or_default()
        ),
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ServiceCommandOutput {
    command: String,
    service_name: String,
    state: &'static str,
    pid: Option<u32>,
}

impl ServiceCommandOutput {
    fn from_report(command: String, report: &ServiceStatusReport) -> Self {
        Self {
            command,
            service_name: report.service_name.as_str().to_string(),
            state: match report.state {
                ServiceState::NotInstalled => "notInstalled",
                ServiceState::Stopped => "stopped",
                ServiceState::Running => "running",
            },
            pid: report.pid,
        }
    }
}
