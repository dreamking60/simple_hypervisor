#![feature(asm)]
#![feature(type_ascription)]
#![no_std]
#![no_main]

use core::arch::global_asm;
use core::fmt::Write;

mod aarch64;
mod panic;

global_asm!(include_str!("start.s"));

#[no_mangle]
fn kmain() {
    print_el();
    hypervisor_main();
}

#[no_mangle]
pub extern "C" fn hypervisor_main() {
    println!("Hypervisor running!");
    print_el();
}

fn print_el() {
    let mut writer = aarch64::uart_console::Writer;
    let c_el = aarch64::asm::get_current_el();
    println!("Current EL: {}", c_el);
}
