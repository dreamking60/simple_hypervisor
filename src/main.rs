#![feature(type_ascription)]
#![no_std]
#![no_main]
#![allow(dead_code)]

use core::arch::{global_asm};

use aarch64::{timer::timer_el2_init};
use aarch64::vm::VM;
use aarch64::regs::Regs;
use aarch64::vcpu::VCPU;

mod aarch64;
mod panic;

global_asm!(include_str!("start.s"));

#[no_mangle]
fn kmain() {
    // Before enable hypervisor
    // check hcr_el2 and exception level
    print_el();
    print_hcr_el2();

    // Enable hypervisor
    aarch64::asm::enable_hypervisor();

    // After enable hypervisor
    // check hcr_el2 and exception level
    print_hcr_el2();
    print_el();
    println!("-------------------init complete-------------------");
    
    // Go to hypervisor main function
    hypervisor_main();
}

#[no_mangle]
pub extern "C" fn hypervisor_main() {
    // start print
    println!("Hypervisor running!");
    print_el();

    enable_interrupts();
    // close the hypervisor
    
}

// print current exception level
fn print_el() {
    let c_el: u64 = aarch64::asm::get_current_el();
    println!("Current EL: {}", c_el);
}

// print hcr_el2 register
fn print_hcr_el2() {
    let hcr_el2: u64 = aarch64::asm::read_hcr_el2();
    println!("REG(HCR_EL2): 0x{:0x}", hcr_el2);
}

// enable interrupts
fn enable_interrupts() {

    timer_el2_init();
    println!("Interrupts enabled");
}

//

// print vcpu registers
fn print_vcpu_regs(vcpu: &VCPU) {
    let regs: &Regs = vcpu.get_regs();
    println!("VCPU: Register Log");

    println!("     x1: 0x{:x}", regs.x1);
    println!("     x2: 0x{:x}", regs.x2);
    println!("     x3: 0x{:x}", regs.x3);
    println!("     x4: 0x{:x}", regs.x4);
    println!("     x5: 0x{:x}", regs.x5);
    println!("     x6: 0x{:x}", regs.x6);
    println!("     x7: 0x{:x}", regs.x7);
    println!("     x8: 0x{:x}", regs.x8);
    println!("     x9: 0x{:x}", regs.x9);
    println!("     x10: 0x{:x}", regs.x10);
    println!("     x11: 0x{:x}", regs.x11);
    println!("     x12: 0x{:x}", regs.x12);
    println!("     x13: 0x{:x}", regs.x13);
    println!("     x14: 0x{:x}", regs.x14);

    println!("VCPU: Register Log End");
}