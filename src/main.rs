#![no_std]
#![no_main]

use core::ops::Add;
use core::{arch::global_asm};
use core::mem::zeroed;

mod panic;
mod uart_console;

global_asm!(include_str!("start.s"));

unsafe fn freebss() {
    extern "C" {
        static mut __bss_beg: u64;
        static mut __bss_end: u64;
    }

    let mut bss_ptr: &mut u64 = &mut __bss_beg;
    let bss_end_ptr: &mut u64 = &mut __bss_end;
    
    while bss_ptr < bss_end_ptr {
        core::ptr::write_volatile(bss_ptr, zeroed());
        bss_ptr = bss_ptr.add(1);
    }

}

#[no_mangle]
pub extern "C" fn hypervisor_main() {
    unsafe {freebss()};

    let mut writer = uart_console::Writer{};
    writer.write_string("Hypervisor running...\r\n");
}
