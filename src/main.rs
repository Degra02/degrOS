#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod test;
mod vga_buffer;
mod serial;
mod utils;

/// First function called at OS startup
#[no_mangle]
pub extern "C" fn _start() -> ! {
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

/// Test runner framework
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    
    exit_qemu(QemuExitCode::Success);   
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

/// Exit Quemu after all the tests are successful
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);    
    }
}
