#![no_std]
#![no_main]

use core::{arch::global_asm};

mod panic;
mod uart_console;

global_asm!(include_str!("start.s"));

#[no_mangle]
pub extern "C" fn hypervisor_main() {
    let mut writer = uart_console::Writer{};
    writer.write_string("Hypervisor running...\r\n");
}
