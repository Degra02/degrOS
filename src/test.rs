use crate::{print, println, vga_buffer::*};

#[test_case]
pub fn test_buffer() {
    let test_str = "Buffer testing";
    let mut i: u8 = 0;
    let mut color_code: ColorCode;
    for c in test_str.bytes() {
        color_code = ColorCode::new(Color::White, Color::from_u8(i));
        WRITER.lock().set_colorcode(color_code);
        WRITER.lock().write_byte(c);
        i += 1;
    }
    WRITER.lock().set_colorcode(ColorCode::new_default());
    println!("...[ok]");
}

#[test_case]
pub fn trivial_assertion() {
    print!("Trivial assertion...");
    assert_eq!(1, 1);
    println!("[ok]");
}
