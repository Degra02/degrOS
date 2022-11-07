#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[cfg(test)]
mod test;
mod vga_buffer;

/// First function called at OS startup
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to {}", "degrOS");
    //panic!("Culo");

    // Only gets compiled if we cargo test
    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // With the println! macro, invoking
    // the panic! macro will print where the code panicked
    println!("{}", _info);
    loop {}
}

/// Test runner framework
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
