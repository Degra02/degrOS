#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"degrOS";
// b"
//  ____  ____  ___  ____   __   ____\n
// (    \\(  __)/ __)(  _ \\ /  \\ / ___)\n
//  ) D ( ) _)( (_ \\ )   /(  O )\\___ \\ \n
// (____/(____)\\___/(__\\_) \\__/ (____/\n";

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

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
