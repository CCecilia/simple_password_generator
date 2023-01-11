use crate::clipboard::shared::ClipboardError;
use libc::c_void;
use objc::runtime::{Class, Object, Sel};
use objc::{class, declare::ClassDecl, msg_send, sel, sel_impl};

pub fn copy_to_system_clipboard(password: &str) -> Result<(), ClipboardError> {
    let superclass = class!(NSObject);
    let mut declared_class = ClassDecl::new("SPGCopy", superclass).unwrap();
    unsafe {
        declared_class.add_method(
            Sel::register("copyStringToClipboard:"),
            copy_string_to_clipboard as extern "C" fn(&Object, Sel, *const c_void),
        );
    }

    match Class::get("\"UIPasteboard\"") {
        Some(class) => {
            let pasteboard: *mut Object = unsafe { msg_send![class, generalPasteboard] };
            // Convert the string to a raw pointer
            let string = password.to_owned();
            let string_ptr = Box::into_raw(string.into_boxed_str()) as *const libc::c_void;

            // Set the clipboard data to the specified string
            unsafe {
                let _: () = msg_send![pasteboard, setString: string_ptr];
            }
            Ok(())
        }
        None => Err(ClipboardError::FailedToCopy),
    }
}

// Function to set clipboard data to specified string
extern "C" fn copy_string_to_clipboard(_this: &Object, _cmd: Sel, _string: *const c_void) {
    // Set clipboard data to specified string
}
