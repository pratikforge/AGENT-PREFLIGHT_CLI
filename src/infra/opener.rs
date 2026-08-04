use std::path::Path;
use std::process::Command;

#[derive(Debug, thiserror::Error)]
pub enum OpenerError {
    #[error("failed to open file '{0}': {1}")]
    SpawnFailed(String, std::io::Error),
    #[error("opener process failed with status {0}")]
    ProcessFailed(std::process::ExitStatus),
}

/// Opens a file in the user's IDE if available, otherwise falls back to the
/// system default application.
///
/// On all platforms it first tries `code` (VS Code / Cursor). If that is not on
/// PATH it falls back to `cmd /C start` (Windows), `open` (macOS), or
/// `xdg-open` (Linux).
pub fn open_file(path: &Path) -> Result<(), OpenerError> {
    let path_str = path.to_string_lossy().to_string();

    // Try VS Code / Cursor first
    if try_ide_open(&path_str) {
        return Ok(());
    }

    // Fallback to system default
    open_system_default(&path_str)
}

/// Attempts to open the file with `code` (VS Code). Returns true on success.
fn try_ide_open(path_str: &str) -> bool {
    Command::new("code")
        .arg(path_str)
        .spawn()
        .and_then(|mut c| c.wait())
        .map(|s| s.success())
        .unwrap_or(false)
}

fn open_system_default(path_str: &str) -> Result<(), OpenerError> {
    #[cfg(target_os = "windows")]
    let mut command = {
        let mut c = Command::new("cmd");
        c.args(["/C", "start", "", path_str]);
        c
    };

    #[cfg(target_os = "macos")]
    let mut command = {
        let mut c = Command::new("open");
        c.arg(path_str);
        c
    };

    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    let mut command = {
        let mut c = Command::new("xdg-open");
        c.arg(path_str);
        c
    };

    let status = command
        .spawn()
        .map_err(|e| OpenerError::SpawnFailed(path_str.to_string(), e))?
        .wait()
        .map_err(|e| OpenerError::SpawnFailed(path_str.to_string(), e))?;

    if !status.success() {
        return Err(OpenerError::ProcessFailed(status));
    }

    Ok(())
}
