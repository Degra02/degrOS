#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(degrOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[no_mangle] 
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    degrOS::test_panic_handler(_info)
}


use degrOS::{print, println, vga_buffer::*, serial_print, serial_println};

pub fn ok() {
    serial_print!("...[");
     WRITER.lock().set_colorcode(
        ColorCode::new(Color::Green, Color::Black)   
    );
    serial_print!("OK");
    WRITER.lock().set_colorcode(ColorCode::new_default());
    serial_println!("]");
}

#[test_case]
fn test_println() {
    println!("testing simple vga buffer println output");
} 

#[test_case]
fn test_buffer_overflow() {
    for i in 0..150 {
        println!("testing buffer overflow");
    }
}

#[test_case]
fn test_println_output_eq() {
    let s = "testing vga buffer eq";
    WRITER.lock().clear_all();
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().get_buffer_mut().get_chars_mut()[0][i].read();
        assert_eq!(char::from(screen_char.get_ascii_character()), c);
    }
    
}

#[test_case]
pub fn test_buffer() {
    let test_str = "Buffer testing";
    let mut color_code: ColorCode;
    for (i, c) in test_str.bytes().enumerate() {
        color_code = ColorCode::new(Color::White, Color::from_u8(i as u8));
        WRITER.lock().set_colorcode(color_code);
        WRITER.lock().write_byte(c);
    }
    WRITER.lock().set_colorcode(ColorCode::new_default());
}

#[test_case]
pub fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[test_case]
pub fn your_mother() {
    println!("your mother");    
}