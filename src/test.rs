use crate::vga_buffer::*;

pub fn test_buffer() {
    let cc = ColorCode::new(Color::White, Color::Black);

    let mut writer = Writer::new(0 as usize, 0 as usize, cc);

    writer.write_string("Welcome to degrOS!");
    writer.write_byte(b'\n');
    writer.write_string("Currently you can't do anything!");
    // write!(writer, "Welcome to degrOS!").unwrap();
}
