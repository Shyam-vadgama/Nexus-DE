use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HyprlandIpcError {
    #[error("HYPRLAND_INSTANCE_SIGNATURE environment variable not found")]
    NoInstanceSignature,
    #[error("XDG_RUNTIME_DIR environment variable not found")]
    NoRuntimeDir,
    #[error("Socket IO error: {0}")]
    Io(#[from] std::io::Error),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socket_paths_missing_env() {
        // Without HYPRLAND_INSTANCE_SIGNATURE, it should return error safely
        std::env::remove_var("HYPRLAND_INSTANCE_SIGNATURE");
        assert!(get_command_socket_path().is_err());
        assert!(get_event_socket_path().is_err());
    }
}
