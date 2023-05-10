#!/bin/bash
qemu-system-aarch64 \
-M virt \
-cpu cortex-a57 \
-machine virtualization=on \
-m 4G \
-nographic \
-kernel target/aarch64-unknown-none/debug/simple_hypervisor
