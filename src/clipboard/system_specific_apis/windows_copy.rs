use crate::clipboard::shared::ClipboardError;
use scopeguard::defer;
use std::ptr;
use winapi::shared::minwindef::FALSE;
use winapi::um::winbase::{GlobalAlloc, GlobalFree, GlobalLock, GlobalUnlock, GMEM_MOVEABLE};
use winapi::um::winuser::{CloseClipboard, OpenClipboard, SetClipboardData, CF_UNICODETEXT};

pub fn copy_to_system_clipboard(password: &str) -> Result<(), ClipboardError> {
    let mut password_utf16: Vec<u16> = password.encode_utf16().collect();
    password_utf16.push(0);

    let hglob = unsafe {
        GlobalAlloc(
            GMEM_MOVEABLE,
            password_utf16.len() * std::mem::size_of::<u16>(),
        )
    };
    if hglob == ptr::null_mut() {
        return Err(ClipboardError::FailedToCopy);
    }
    defer!(unsafe { GlobalFree(hglob) };);

    let dst = unsafe { GlobalLock(hglob) };
    if dst == ptr::null_mut() {
        return Err(ClipboardError::FailedToCopy);
    }

    unsafe { ptr::copy_nonoverlapping(password_utf16.as_ptr(), dst as _, password_utf16.len()) };
    unsafe { GlobalUnlock(hglob) };

    let success = unsafe { OpenClipboard(ptr::null_mut()) } != FALSE;
    if !success {
        return Err(ClipboardError::FailedToCopy);
    }
    defer!(unsafe { CloseClipboard() };);

    let success = unsafe { SetClipboardData(CF_UNICODETEXT, hglob) } != ptr::null_mut();
    if !success {
        return Err(ClipboardError::FailedToCopy);
    }

    Ok(())
}
