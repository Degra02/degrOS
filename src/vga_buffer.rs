use volatile::Volatile;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

impl Color {
    pub fn from_u8(i: u8) -> Color {
        match i {
            0 => Color::Black,
            1 => Color::Blue,
            2 => Color::Green,
            3 => Color::Cyan,
            4 => Color::Red,
            5 => Color::Magenta,
            6 => Color::Brown,
            7 => Color::LightGray,
            8 => Color::DarkGray,
            9 => Color::LightBlue,
            10 => Color::LightGreen,
            11 => Color::LightCyan,
            12 => Color::LightRed,
            13 => Color::Pink,
            14 => Color::Yellow,
            15 => Color::White,
            _ => Color::Black,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    /// Creates a new [`ColorCode`].
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }

    /// New ColorCode with default values
    pub fn new_default() -> ColorCode {
        ColorCode((Color::Black as u8) << 4 | (Color::White as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
/// Single character represented by an ascii code
/// and a color code
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

impl ScreenChar {
    pub fn new(ascii_character: u8, color_code: ColorCode) -> ScreenChar {
        ScreenChar {
            ascii_character,
            color_code,
        }
    }
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
/// Text Buffer
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// Struct for the Write To Buffer
pub struct Writer {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

use core::fmt;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

impl Writer {
    /// Creates a new Writer instance from column position, row position
    /// and ColorCode
    pub fn new(cpos: usize, rpos: usize, cc: ColorCode) -> Writer {
        Writer {
            column_position: cpos,
            row_position: rpos,
            color_code: cc.clone(),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    }

    /// Change the Writer color code
    pub fn set_colorcode(&mut self, cc: ColorCode) {
        self.color_code = cc;
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_position;
                let column = self.column_position;
                let color_code = self.color_code;

                self.buffer.chars[row][column].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });

                self.column_position += 1;
            }
        }
    }

    /// Writes a string by converting it to bytes
    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                // Printable character or newline:
                0x20..=0x7e => self.write_byte(byte),

                // Detects newline char
                0x0A => self.write_byte(b'\n'),

                // Fallback character => ■
                _ => self.write_byte(0xfe),
            }
        }
    }

    /// Goes to new line
    fn new_line(&mut self) {
        if self.row_position < BUFFER_HEIGHT {
            self.row_position += 1;
        } else {
            self.row_position = 0;
        }
        self.column_position = 0;
    }

    fn clear_all(&mut self, row: usize) {
        let blank = ScreenChar::new(b' ', self.color_code);
        for line in 0..BUFFER_HEIGHT - 1 {
            for col in 0..BUFFER_WIDTH - 1 {
                self.buffer.chars[line][col].write(blank);
            }
        }
    }
}

/// WRITER so it doesn't have to be
/// instantiated each time it needs to be used
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        row_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

// Macros for quicker printing (errors too)
#[macro_export]
///Directly prints to the static WRITER
macro_rules! print {
    ($($arg:tt)*) => {
        ($crate::vga_buffer::_print(format_args!($($arg)*)))
    }
}

#[macro_export]
/// Directly prints to the static WRITER appending 
/// a newline at the end
macro_rules! println {
    () => {
        $crate::print!("\n");
    };

    ($($arg:tt)*) => {
      ($crate::print!("{}\n", format_args!($($arg)*)))
    };
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
