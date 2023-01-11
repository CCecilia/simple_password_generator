#[cfg(target_os = "macos")]
use crate::clipboard::system_specific_apis::ios_copy::copy_to_system_clipboard;
#[cfg(target_os = "linux")]
use crate::clipboard::system_specific_apis::linux_copy::copy_to_system_clipboard;
#[cfg(target_os = "windows")]
use crate::clipboard::system_specific_apis::windows_copy::copy_to_system_clipboard;
use std::{error::Error, fmt};

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

pub fn copy_password_to_clipboard(password: &str) -> Result<(), ClipboardError> {
    copy_to_system_clipboard(password)
}
