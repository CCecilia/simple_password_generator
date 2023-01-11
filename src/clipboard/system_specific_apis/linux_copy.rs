use crate::clipboard::shared::ClipboardError;
use std::io::Write;
use std::process::{Command, Stdio};

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

pub fn copy_to_system_clipboard(password: &str) -> Result<(), ClipboardError> {
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
