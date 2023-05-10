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
    print_hcr_el2();
    aarch64::asm::enable_hypervisor();
    print_hcr_el2();
    print_el();
    hypervisor_main();
}

#[no_mangle]
pub extern "C" fn hypervisor_main() {
    println!("Hypervisor running!");
    print_el();

    // close the hypervisor

}

fn print_el() {
    let c_el: u64 = aarch64::asm::get_current_el();
    println!("Current EL: {}", c_el);
}

fn print_hcr_el2() {
    let hcr_el2: u64 = aarch64::asm::read_hcr_el2();
    println!("REG(HCR_EL2): 0x{:0x}", hcr_el2);
}
