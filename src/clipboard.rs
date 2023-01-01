use scopeguard::defer;
use std::ptr;
use std::{error::Error, fmt};
use winapi::shared::minwindef::FALSE;
use winapi::um::winbase::{GlobalAlloc, GlobalFree, GlobalLock, GlobalUnlock, GMEM_MOVEABLE};
use winapi::um::winuser::{CloseClipboard, OpenClipboard, SetClipboardData, CF_UNICODETEXT};

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

#[cfg(target_os = "linux")]
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

#[cfg(target_os = "windows")]
pub fn copy_to_clipboard(password: &str) -> Result<(), ClipboardError> {
    // Needs to be UTF-16 encoded
    let mut password_utf16: Vec<u16> = password.encode_utf16().collect();
    // And zero-terminated before passing it into `SetClipboardData`
    password_utf16.push(0);
    // Allocate memory
    let hglob = unsafe {
        GlobalAlloc(
            GMEM_MOVEABLE,
            password_utf16.len() * std::mem::size_of::<u16>(),
        )
    };
    if hglob == ptr::null_mut() {
        return Err(ClipboardError::FailedToCopy);
    }
    // Ensure cleanup on scope exit
    defer!(unsafe { GlobalFree(hglob) };);

    // Retrieve writeable pointer to memory
    let dst = unsafe { GlobalLock(hglob) };
    if dst == ptr::null_mut() {
        return Err(ClipboardError::FailedToCopy);
    }
    // Copy data
    unsafe { ptr::copy_nonoverlapping(password_utf16.as_ptr(), dst as _, password_utf16.len()) };
    // Release writeable pointer
    unsafe { GlobalUnlock(hglob) };

    // Everything is set up now, let's open the clipboard
    let success = unsafe { OpenClipboard(ptr::null_mut()) } != FALSE;
    if !success {
        return Err(ClipboardError::FailedToCopy);
    }
    // Ensure cleanup on scope exit
    defer!(unsafe { CloseClipboard() };);
    // And apply data
    let success = unsafe { SetClipboardData(CF_UNICODETEXT, hglob) } != ptr::null_mut();
    if !success {
        return Err(ClipboardError::FailedToCopy);
    }

    Ok(())
}

#[cfg(target_os = "macos")]
pub fn copy_to_clipboard(s: &str) {
    // macOS-specific clipboard code goes here
}
