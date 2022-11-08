use crate::{print, println, vga_buffer::*, serial_print, serial_println};

pub fn ok() {
    serial_print!("...[");
     WRITER.lock().set_colorcode(
        ColorCode::new(Color::Green, Color::Black)   
    );
    serial_print!("OK");
    WRITER.lock().set_colorcode(ColorCode::new_default());
    serial_println!("]");
}


// #[test_case]
pub fn test_buffer() {
    let test_str = "Buffer testing";
    let mut i: u8 = 0;
    let mut color_code: ColorCode;
    for c in test_str.bytes() {
        color_code = ColorCode::new(Color::White, Color::from_u8(i));
        WRITER.lock().set_colorcode(color_code);
        // serial_print!(c);
        i += 1;
    }
    WRITER.lock().set_colorcode(ColorCode::new_default());
}

#[test_case]
pub fn serial_test_buffer() {
    // serial_print!("VGA buffer testing");
    // println!("\n");
}

#[test_case]
pub fn trivial_assertion() {
    // serial_print!("Trivial assertion");
    assert_eq!(1, 1);
}

#[test_case]
pub fn your_mother() {
    // serial_print!("Your mother");
}
