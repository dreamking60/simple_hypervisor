#![no_std]
#![no_main]

use core::{arch::global_asm, fmt::Write};

mod panic;

global_asm!(include_str!("start.S"));
include!("uart_console.rs");

#[no_mangle]
pub extern "C" fn hypervisor_main() {
    print_something();
}

pub fn print_something() {
    let mut writer = Writer {};

    // Test the two functions we implemented for Writer
    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!\n");

    writer.write_string("[0] Hello from Rust!\n");

    // Verify the auto-implemented methods from core::fmt::Write
    let display = format_args!("hello arguments!\n");
    writer.write_fmt(display).unwrap();

    // Use the write! macro for formatted output
    write!(writer, "The numbers are {} and {} \n", 42, 1.0 / 3.0).unwrap();
}
