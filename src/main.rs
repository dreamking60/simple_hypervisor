#![no_std]
#![no_main]

use core::{ptr, arch::global_asm};
mod panic;
//mod uart_console;

global_asm!(include_str!("start.S"));

// #[no_mangle]
// pub extern "C" fn hypervisor_main() {
//     let mut writer = uart_console::Writer;
//     writer.write_string("Hypervisor Start\n");
// }

#[no_mangle] // 不修改函数名
pub extern "C" fn hypervisor_main() {
    print_something();
}

//以下是测试代码部分
include!("uart_console.rs");
//引用Writer需要的控件
use core::fmt;
use core::fmt::Write;

//测试函数
pub fn print_something() {
    let mut writer = Writer{};

    // 测试Writer我们实现的两个函数
    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
    writer.write_string("[0] Hello from Rust!");

    // 验证实现core::fmt::Write自动实现的方法
    let display: fmt::Arguments = format_args!("hello arguments!");
    writer.write_string("Start!");
    writer.write_fmt(display).unwrap();
    // 使用write!宏进行格式化输出
    write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();

    writer.write_string("Complete!");
}
