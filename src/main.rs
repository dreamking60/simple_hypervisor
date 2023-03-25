#![no_std]
#![no_main]

use core::{arch::global_asm};

mod panic;
mod uart_console;

global_asm!(include_str!("start.S"));

#[no_mangle]
pub extern "C" fn hypervisor_main() {
    print_something();
}

pub fn print_something() {
    println!("Hello, world!\n");
}
