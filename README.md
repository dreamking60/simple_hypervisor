The cargo command
```
cargo xbuild --target=aarch64-unknown-none.json [--release]
```

The qemu command
```
qemu-system-aarch64 -machine virt -m 1024M -cpu cortex-a53 -nographic -kernel target/aarch64-unknown-none/debug/simple_hypervisor

qemu-system-aarch64 -machine virt -m 1024M -cpu cortex-a53 -nographic -kernel target/aarch64-unknown-none/release/simple_hypervisor

qemu-system-aarch64 -machine virt -m 1024M -cpu cortex-a53 -nographic -kernel target/aarch64-unknown-none/debug/simple_hypervisor -S -s

```