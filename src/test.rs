use crate::{print, println, vga_buffer::*};

fn ok() {
    print!("[");
     WRITER.lock().set_colorcode(
        ColorCode::new(Color::Green, Color::Black)   
    );
    print!("OK");
    WRITER.lock().set_colorcode(ColorCode::new_default());
    println!("]");
}


#[test_case]
pub fn test_buffer() {
    let test_str = "Buffer testing...";
    let mut i: u8 = 0;
    let mut color_code: ColorCode;
    for c in test_str.bytes() {
        color_code = ColorCode::new(Color::White, Color::from_u8(i));
        WRITER.lock().set_colorcode(color_code);
        WRITER.lock().write_byte(c);
        i += 1;
    }
    WRITER.lock().set_colorcode(ColorCode::new_default());
    ok();
}

#[test_case]
pub fn trivial_assertion() {
    print!("Trivial assertion...");
    assert_eq!(1, 1);
    ok();
}
