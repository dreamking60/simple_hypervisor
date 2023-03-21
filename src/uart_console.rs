pub struct Writer;

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        unsafe {
            const UART: *mut u8 = 0x0900_0000 as *mut u8;
            core::ptr::write_volatile(UART, byte);
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte as u8);
        }
    }
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}