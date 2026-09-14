use std::path::PathBuf;
use thiserror::Error;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tracing::debug;

#[derive(Debug, Error)]
pub enum HyprlandIpcError {
    #[error("HYPRLAND_INSTANCE_SIGNATURE environment variable not found")]
    NoInstanceSignature,
    #[error("XDG_RUNTIME_DIR environment variable not found")]
    NoRuntimeDir,
    #[error("Socket IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON deserialization error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Retrieve the active Hyprland command socket path (.socket.sock)
pub fn get_command_socket_path() -> Result<PathBuf, HyprlandIpcError> {
    let signature = std::env::var("HYPRLAND_INSTANCE_SIGNATURE")
        .map_err(|_| HyprlandIpcError::NoInstanceSignature)?;
    let runtime_dir =
        std::env::var("XDG_RUNTIME_DIR").map_err(|_| HyprlandIpcError::NoRuntimeDir)?;
    Ok(PathBuf::from(runtime_dir)
        .join("hypr")
        .join(signature)
        .join(".socket.sock"))
}

/// Retrieve the active Hyprland event socket path (.socket2.sock)
pub fn get_event_socket_path() -> Result<PathBuf, HyprlandIpcError> {
    let signature = std::env::var("HYPRLAND_INSTANCE_SIGNATURE")
        .map_err(|_| HyprlandIpcError::NoInstanceSignature)?;
    let runtime_dir =
        std::env::var("XDG_RUNTIME_DIR").map_err(|_| HyprlandIpcError::NoRuntimeDir)?;
    Ok(PathBuf::from(runtime_dir)
        .join("hypr")
        .join(signature)
        .join(".socket2.sock"))
}

/// Send a raw command to Hyprland's command socket and return the response.
pub async fn send_command(cmd: &str) -> Result<String, HyprlandIpcError> {
    let socket_path = get_command_socket_path()?;
    let mut stream = UnixStream::connect(socket_path).await?;
    stream.write_all(cmd.as_bytes()).await?;
    stream.shutdown().await?;

    let mut response = String::new();
    stream.read_to_string(&mut response).await?;
    Ok(response)
}

/// Query Hyprland active workspaces as JSON
pub async fn get_workspaces_json() -> Result<serde_json::Value, HyprlandIpcError> {
    let response = send_command("j/workspaces").await?;
    let json: serde_json::Value = serde_json::from_str(&response)?;
    Ok(json)
}

/// Query Hyprland active window as JSON
pub async fn get_active_window_json() -> Result<serde_json::Value, HyprlandIpcError> {
    let response = send_command("j/activewindow").await?;
    let json: serde_json::Value = serde_json::from_str(&response)?;
    Ok(json)
}

/// Dispatch a Hyprland action (e.g. "workspace 2", "exec alacritty")
pub async fn dispatch(dispatcher: &str, args: &str) -> Result<String, HyprlandIpcError> {
    let cmd = format!("dispatch {} {}", dispatcher, args);
    send_command(&cmd).await
}

/// Async Hyprland event listener connected to .socket2.sock
pub async fn listen_events<F, Fut>(mut handler: F) -> Result<(), HyprlandIpcError>
where
    F: FnMut(String) -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    let socket_path = get_event_socket_path()?;
    let stream = UnixStream::connect(socket_path).await?;
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        debug!("Hyprland event: {}", line);
        handler(line).await;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socket_paths_missing_env() {
        std::env::remove_var("HYPRLAND_INSTANCE_SIGNATURE");
        assert!(get_command_socket_path().is_err());
        assert!(get_event_socket_path().is_err());
    }

    #[tokio::test]
    async fn test_mock_unix_socket_roundtrip() {
        let temp_dir = std::env::temp_dir();
        let socket_path = temp_dir.join("test_hypr_cmd.sock");
        let _ = std::fs::remove_file(&socket_path);

        let listener = tokio::net::UnixListener::bind(&socket_path).expect("Bind mock socket");

        // Spawn mock server
        tokio::spawn(async move {
            if let Ok((mut stream, _)) = listener.accept().await {
                let mut buf = vec![0u8; 128];
                let n = stream.read(&mut buf).await.unwrap();
                let cmd = String::from_utf8_lossy(&buf[..n]);
                if cmd.starts_with("j/workspaces") {
                    let response = r#"[{"id":1,"name":"1"}]"#;
                    stream.write_all(response.as_bytes()).await.unwrap();
                }
            }
        });

        // Test reading from mock socket
        let mut client = UnixStream::connect(&socket_path)
            .await
            .expect("Connect to mock");
        client.write_all(b"j/workspaces").await.unwrap();
        client.shutdown().await.unwrap();

        let mut res = String::new();
        client.read_to_string(&mut res).await.unwrap();
        assert_eq!(res, r#"[{"id":1,"name":"1"}]"#);

        let _ = std::fs::remove_file(&socket_path);
    }
}
