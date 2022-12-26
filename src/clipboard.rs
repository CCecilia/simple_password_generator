use std::io::Write;
use std::{
    error::Error,
    fmt,
    process::{Command, Stdio},
};

#[derive(Debug)]
pub enum ClipboardError {
    NoXclip,
    FailedToCopy,
}

impl Error for ClipboardError {}

impl fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClipboardError::NoXclip => write!(f, "XClip not found"),
            ClipboardError::FailedToCopy => write!(f, "Failed to copy"),
        }
    }
}

fn check_for_xclip() -> Result<(), ClipboardError> {
    let cmd = Command::new("which").arg("xclip").output();

    if cmd.is_ok() {
        let result = cmd.unwrap();
        if result.stdout.is_empty() {
            return Err(ClipboardError::NoXclip);
        }
    };

    Ok(())
}

#[cfg(target_os = "windows")]
fn copy_to_clipboard(s: &str) {
    // Windows-specific clipboard code goes here
}

#[cfg(target_os = "macos")]
fn copy_to_clipboard(s: &str) {
    // macOS-specific clipboard code goes here
}

#[cfg(target_os = "linux")]
pub fn copy_to_clipboard(password: &str) -> Result<(), ClipboardError> {
    if let Err(e) = check_for_xclip() {
        return Err(e);
    }

    let xclip_process_spawn = Command::new("xclip")
        .arg("-selection")
        .arg("clipboard")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn();

    let copy = match xclip_process_spawn {
        Ok(mut child_process) => {
            let stdin = child_process.stdin.as_mut();
            let output = match stdin {
                Some(child_stdin) => child_stdin.write_all(password.as_bytes()),
                None => return Err(ClipboardError::FailedToCopy),
            };

            output
        }
        Err(_e) => return Err(ClipboardError::FailedToCopy),
    };

    if copy.is_err() {
        return Err(ClipboardError::FailedToCopy);
    }

    Ok(())
}
