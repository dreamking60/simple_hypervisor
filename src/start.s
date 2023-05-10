.global _start
.extern LD_STACK_PTR

.section ".text.boot"

_start:
    ldr     x30, =LD_STACK_PTR
    mov     sp, x30
    bl      kmain

.equ PSCI_SYSTEM_OFF, 0x84000008
.global system_off
system_off:
    ldr     x0, =PSCI_SYSTEM_OFF
    hvc     #0