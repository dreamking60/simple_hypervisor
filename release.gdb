set disassemble-next-line on
set confirm off
add-symbol-file target/aarch64-unknown-none/release/simple_hypervisor
target remote tcp::1234
set arch aarch64
layout regs
