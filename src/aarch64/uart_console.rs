use core::fmt;
use core::fmt::Write;
use core::ptr;

pub struct Writer;

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

    pub fn write_number(&mut self, num: u64) {
        let mut buf: [u8; 20] = [0u8; 20];
        let mut i: usize = 0;
        let mut n: u64 = num;
        loop {
            buf[i] = (n % 10) as u8 + b'0';
            n /= 10;
            i += 1;
            if n == 0 {
                break;
            }
        }
        for c in buf[..i].iter().rev() {
            self.write_byte(*c);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Writer.write_fmt(args).unwrap();
}

// print! macro
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::aarch64::uart_console::print(format_args!($($arg)*)));
}

// println! macro
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => ($crate::print!("{}\r\n", format_args!($($arg)*)));
}