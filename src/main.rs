#![no_std]  // We can't rely on the standard library
#![no_main] // Disable entry point

use core::panic::PanicInfo;

/// Required for no_std applications
/// See https://doc.rust-lang.org/nomicon/panic-handler.html
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// #[no_mangle] prevents function renaming so we can link to it.
/// extern "C" means we use the C calling convention.
///
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
