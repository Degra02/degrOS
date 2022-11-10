#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(degrOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use degrOS::{serial_println, exit_qemu};
use degrOS::QemuExitCode;
pub mod utils;
pub mod vga_buffer;

/// First function called at OS startup
#[no_mangle]
pub extern "C" fn _start() -> ! {
    utils::startup_message();
    
    #[cfg(test)]
    test_main();
    
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}


#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error {}", _info);
    exit_qemu(QemuExitCode::Failed);
    loop{}
}
