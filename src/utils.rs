use crate::{print, println};

/// Sending the default startup message
pub fn startup_message() {
    use crate::vga_buffer::{WRITER, ColorCode, Color};

    print!("Starting up ");
    WRITER.lock().set_colorcode(
        ColorCode::new(Color::Cyan, Color::Black)   
    );
    println!("degrOS");
    WRITER.lock().set_colorcode(ColorCode::new_default());
}
