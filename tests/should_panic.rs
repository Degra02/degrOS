#![no_std]
#![no_main]

use core::panic::PanicInfo;
use degrOS::{QemuExitCode, exit_qemu, serial_print, serial_println};


// In Cargo.toml, 'harness' is disabled for this file so 
// the test can be run without a test_runner
// Disabling the 'harness' attribute can be useful also 
// complex integration tests


#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[Test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop{}
}

// Test that should fail
fn should_fail() {
    serial_print!("should_panic::should_fail\t");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("...[OK]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

