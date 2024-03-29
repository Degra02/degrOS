#![allow(non_snake_case)]
#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod utils;
pub mod vga_buffer;

// We need this lib.rs in order to make these functions public
// to the integration tests

/// Trait specifying that a Fn is a test function
/// and implementing automatic printing
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) -> () {
        // Printing the test function name (type_name)
        serial_print!("{: <25}", core::any::type_name::<T>());
        self(); // Running the test function
        serial_println!("...[OK]")
    }
}

/// Test runner framework
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(_info: &PanicInfo) -> ! {
    serial_println!("...[failed]\n");
    serial_println!("Error {}", _info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(test_kernel_main);

/// Entry point for cargo test
#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    utils::serial_startup_message();
    test_main();

    hlt_loop();
}

pub fn init() {
    interrupts::init_idt(); // Initializing interrupt-descriptor-table
    gdt::init(); // Initializing global-descriptor-table
    unsafe { interrupts::PICS.lock().initialize() } // Initializing
                                                    // Programmable-Interrupt-Controllers
    x86_64::instructions::interrupts::enable();
}

/// Halt the CPU until the next interrupt;
/// CU enters a sleep state, consuming less energy.
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    test_panic_handler(_info)
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
