#![allow(dead_code)]
use crate::{serial_print, serial_println};

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
        serial_print!("{}\t", core::any::type_name::<T>());
        self(); // Running the test function
        super::tests::ok();
    }
}

/// Test runner framework
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
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
    // use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
