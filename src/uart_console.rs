// Import core::fmt::Write for write! macro
use core::fmt;
use core::ptr;

// Implement a custom writer for UART
use lazy_static::lazy_static;
use spin::Mutex;

pub struct Writer;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer { });
}

// implement write function for Writer
impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        const UART0: *mut u8 = 0x0900_0000 as *mut u8;
        unsafe {
            ptr::write_volatile(UART0, byte);
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.chars() {
            self.write_byte(byte as u8)
        }
    }
}

// Implement core::fmt::Write for Writer
// impl core::fmt::Write for Writer {
//     fn write_str(&mut self, s: &str) -> fmt::Result {
//         self.write_string(s);
//         Ok(())
//     }
//   }

// Define a custom println! macro for no_std
// #[doc(hidden)]
// pub fn _print(args: fmt::Arguments) {
//     use core::fmt::Write;
//     WRITER.lock().write_fmt(args).unwrap();
// }

// #[macro_export]
// macro_rules! print {
//     ($($arg:tt)*) => ($crate::uart_console::_print(format_args!($($arg)*)));
// }

// #[macro_export]
// macro_rules! println {
//     () => ($crate::print!("\n"));
//     ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
// }