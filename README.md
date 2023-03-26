# Configuration
rustup 1.25.2 (17db695f1 2023-02-01)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.70.0-nightly (8be3c2bda 2023-03-24)`

rustc 1.70.0-nightly (8be3c2bda 2023-03-24)

cargo 1.70.0-nightly (15d090969 2023-03-21)

# Compile
```
cargo xbuild --target=aarch64-unknown-none.json [--release]
```

# Run
```bash
qemu-system-aarch64 -machine virt -m 1024M -cpu cortex-a53 -nographic -kernel target/aarch64-unknown-none/debug/simple_hypervisor
```

```bash
qemu-system-aarch64 -machine virt -m 1024M -cpu cortex-a53 -nographic -kernel target/aarch64-unknown-none/release/simple_hypervisor
```

```bash
qemu-system-aarch64 -machine virt -m 1024M -cpu cortex-a53 -nographic -kernel target/aarch64-unknown-none/debug/simple_hypervisor -S -s
```