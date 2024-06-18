use crate::{
    lean_box, lean_mk_string_from_bytes, lean_panic_fn, lean_set_exit_on_panic,
    lean_set_panic_messages,
};

/// A panic handler implementation. This is useful when
/// writing FFI libraries for Lean, where people may want to disable rust's `std`.
/// ```ignore
/// #![no_std]
/// #[panic_handler]
/// fn panic(info: &core::panic::PanicInfo) -> ! {
///    lean_sys::panic::panic_handler(info)
/// }
/// ```
pub fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    #[allow(deprecated)] // the payload() function always returns None
    let message = if let Some(s) = info.payload().downcast_ref::<&str>() {
        s.as_bytes()
    } else {
        b"panic inside Rust FFI, no message available"
    };
    unsafe {
        let string = lean_mk_string_from_bytes(message.as_ptr(), message.len());
        loop {
            lean_set_panic_messages(true);
            lean_set_exit_on_panic(true);
            lean_panic_fn(lean_box(0), string);
        }
    }
}
