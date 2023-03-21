#![no_std]
#![no_main]

use core::str::Bytes;
use core::{arch::global_asm};
use core::fmt;
use core::fmt::Write;

mod panic;

global_asm!(include_str!("start.S"));
include!("uart_console.rs");

#[no_mangle]
pub extern "C" fn hypervisor_main() {
    print_something();
}   

pub fn print_something() {
    let mut writer = Writer{};

    // 测试Writer我们实现的两个函数
    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!\n");
    
    writer.write_string("[0] Hello from Rust!\n");

    // 验证实现core::fmt::Write自动实现的方法
    let display: fmt::Arguments = format_args!("hello arguments!\n");
    writer.write_fmt(display).unwrap();
    // 使用write!宏进行格式化输出
    write!(writer, "The numbers are {} and {} \n", 42, 1.0/3.0).unwrap();
}