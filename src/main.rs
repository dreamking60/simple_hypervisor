#![feature(asm)]
#![feature(type_ascription)]
#![no_std]
#![no_main]

use core::arch::{global_asm};

mod aarch64;
mod panic;

global_asm!(include_str!("start.s"));

#[no_mangle]
fn kmain() {
    print_el();
    //aarch64::asm::enable_hypervisor();
    print_el();
    hypervisor_main();
}

#[no_mangle]
pub extern "C" fn hypervisor_main() {
    println!("Hypervisor running!");
    print_el();
}

fn print_el() {
    let c_el: u32 = aarch64::asm::get_current_el();
    println!("Current EL: {}", c_el);
}
