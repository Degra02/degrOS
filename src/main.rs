#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::tests_core::test_runner)]
#![reexport_test_harness_main = "test_main"]


use crate::tests::tests_core::{exit_qemu, QemuExitCode};
use core::panic::PanicInfo;

mod tests;
mod vga_buffer;
mod serial;
mod utils;

/// First function called at OS startup
#[no_mangle]
pub extern "C" fn _start() -> ! {
    utils::startup_message();
    utils::serial_startup_message();
    
    #[cfg(test)]
    test_main();
    
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // With the println! macro, invoking
    // the panic! macro will print where the code panicked
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
