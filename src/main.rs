#![no_std]  // We can't rely on the standard library
#![no_main] // Disable entry point

use core::panic::PanicInfo;

/// Required for no_std applications
/// See https://doc.rust-lang.org/nomicon/panic-handler.html
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

/// #[no_mangle] prevents function renaming so we can link to it.
/// extern "C" means we use the C calling convention.
///
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
