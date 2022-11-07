#![no_std]
#![no_main]

mod test;
mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // test::test_buffer();

    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Welcome to ").unwrap();
    write!(vga_buffer::WRITER.lock(), "{}!\n Culone", "degrOS").unwrap();
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
