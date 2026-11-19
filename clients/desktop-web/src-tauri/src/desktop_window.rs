//! Keep the macOS desktop window separate from the menu bar companion.

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;
use std::time::Duration;
use tauri::Manager;
use tokio::io::AsyncReadExt;

pub(super) struct DesktopWindowInstance {
    _lock: File,
    listener: Option<TcpListener>,
    token: String,
    control_path: PathBuf,
}

impl DesktopWindowInstance {
    pub(super) fn acquire() -> Result<Option<Self>, String> {
        let directory = super::engine_data_directory()?;
        std::fs::create_dir_all(&directory).map_err(|error| error.to_string())?;
        let lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .mode(0o600)
            .open(directory.join("desktop-window.lock"))
            .map_err(|error| error.to_string())?;
        let control_path = directory.join("desktop-window-control.json");
        match fs2::FileExt::try_lock_exclusive(&lock) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                // Another launch may still be publishing its endpoint.
                for _ in 0..40 {
                    if forward_open(&control_path).is_ok() {
                        return Ok(None);
                    }
                    std::thread::sleep(Duration::from_millis(50));
                }
                return Err("MSC is already opening but could not receive the window request. Try opening it again.".into());
            }
            Err(error) => return Err(error.to_string()),
        }
        let listener =
            TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).map_err(|error| error.to_string())?;
        listener
            .set_nonblocking(true)
            .map_err(|error| error.to_string())?;
        let token = uuid::Uuid::new_v4().to_string();
        let port = listener
            .local_addr()
            .map_err(|error| error.to_string())?
            .port();
        let bytes = serde_json::to_vec(&serde_json::json!({ "port": port, "token": token }))
            .map_err(|error| error.to_string())?;
        let mut control = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .mode(0o600)
            .open(&control_path)
            .map_err(|error| error.to_string())?;
        control
            .write_all(&bytes)
            .map_err(|error| error.to_string())?;
        Ok(Some(Self {
            _lock: lock,
            listener: Some(listener),
            token,
            control_path,
        }))
    }

    pub(super) fn attach(mut self, app: &mut tauri::App) {
        let listener = self
            .listener
            .take()
            .expect("desktop listener is attached once");
        let token = self.token.clone();
        let handle = app.handle().clone();
        app.manage(self);
        tauri::async_runtime::spawn(async move {
            let Ok(listener) = tokio::net::TcpListener::from_std(listener) else {
                return;
            };
            while let Ok((mut stream, _)) = listener.accept().await {
                let mut supplied = vec![0; token.len()];
                if matches!(
                    tokio::time::timeout(Duration::from_secs(1), stream.read_exact(&mut supplied))
                        .await,
                    Ok(Ok(_))
                ) && supplied == token.as_bytes()
                {
                    show(&handle);
                }
            }
        });
    }
}

impl Drop for DesktopWindowInstance {
    fn drop(&mut self) {
        // Remove the endpoint while the instance lock is still held.
        let _ = std::fs::remove_file(&self.control_path);
    }
}

fn forward_open(path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let value: serde_json::Value = serde_json::from_slice(&std::fs::read(path)?)?;
    let port = value["port"]
        .as_u64()
        .and_then(|port| u16::try_from(port).ok())
        .ok_or("invalid desktop window port")?;
    let token = value["token"]
        .as_str()
        .ok_or("missing desktop window token")?;
    let address = SocketAddr::from((Ipv4Addr::LOCALHOST, port));
    let mut stream = TcpStream::connect_timeout(&address, Duration::from_millis(200))?;
    stream.set_write_timeout(Some(Duration::from_secs(1)))?;
    stream.write_all(token.as_bytes())?;
    Ok(())
}

pub(super) fn show(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

pub(super) fn open() {
    if let Ok(executable) = std::env::current_exe() {
        if let Err(error) = std::process::Command::new(executable).spawn() {
            eprintln!("Could not open MSC: {error}");
        }
    }
}
