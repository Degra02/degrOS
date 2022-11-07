use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        // Standard port number for the first serial interface => 0x3F8
        // Then it calculates the address of all the other ports
        // on its own
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Printing to SERIAL1 failed");
}


/// Prints to the HOST through SERIAL INTERFACE
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*))
    };
}

/// Prints and adds newline char to the 
/// HOST through SERIAL INTERFACE
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    
    ($fmt:expr) => {
        $crate::serial_print!(concat!($fmt, "\n"))
    };
    
    ($fmt:expr, $($arg:tt)*) => {
        $crate::serial_print!(
            concat!($fmt, "\n"), $($arg)*);
    };
}