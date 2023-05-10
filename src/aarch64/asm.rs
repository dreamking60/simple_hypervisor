use core::arch::asm;
// Wait for event
#[inline(always)]
pub fn wfe() {
    unsafe { asm!("wfe") };
}

// Wait for interrupt
#[inline(always)]
pub fn wfi() {
    unsafe { asm!("wfi") };
}

// Nop
#[inline(always)]
pub fn nop() {
    unsafe { asm!("nop") };
}

// Transition to a lower level
#[inline(always)]
pub fn eret() {
    unsafe { asm!("eret") };
}

// Send event
#[inline(always)]
pub fn sev() {
    unsafe { asm!("sev") };
}

// Send event to all cores
#[inline(always)]
pub fn sevl() {
    unsafe { asm!("sevl") };
}

// Enable interrupts
#[inline(always)]
pub fn enable_irq() {
    unsafe { asm!("msr DAIFClr, #2") };
}

// Disable interrupts
#[inline(always)]
pub fn disable_irq() {
    unsafe { asm!("msr DAIFSet, #2") };
}

// Enable FIQ
#[inline(always)]
pub fn enable_fiq() {
    unsafe { asm!("msr DAIFClr, #1") };
}

// Disable FIQ
#[inline(always)]
pub fn disable_fiq() {
    unsafe { asm!("msr DAIFSet, #1") };
}

// Enable debug exceptions
#[inline(always)]
pub fn enable_dbg() {
    unsafe { asm!("msr DAIFClr, #4") };
}

// Disable debug exceptions
#[inline(always)]
pub fn disable_dbg() {
    unsafe { asm!("msr DAIFSet, #4") };
}

// Enable SError interrupt
#[inline(always)]
pub fn enable_serror() {
    unsafe { asm!("msr DAIFClr, #8") };
}

// Disable SError interrupt
#[inline(always)]
pub fn disable_serror() {
    unsafe { asm!("msr DAIFSet, #8") };
}

// read register
#[inline(always)]
pub fn read_reg(reg: u64) -> u64 {
    let val: u64;
    unsafe {
        asm!("mov {}, {}", out(reg) val, in(reg) reg, options(nomem, nostack, preserves_flags));
    }
    val
}

// get current exception level
#[inline(always)]
pub fn get_current_el() -> u32 {
    let current_el: u64;
    unsafe {
        asm!("mrs {}, CurrentEL", out(reg) current_el, options(nomem, nostack, preserves_flags));
    }
    (current_el >> 2) as u32
}

// read elr_el2 register
#[inline(always)]
pub fn read_elr_el2() -> u64 {
    let elr_el2: u64;
    unsafe {
        asm!("mrs {}, ELR_EL2", out(reg) elr_el2, options(nomem, nostack, preserves_flags));
    }
    elr_el2
}

// read elr_el1 register
#[inline(always)]
pub fn read_elr_el1() -> u64 {
    let elr_el1: u64;
    unsafe {
        asm!("mrs {}, ELR_EL1", out(reg) elr_el1, options(nomem, nostack, preserves_flags));
    }
    elr_el1
}

// read spsr_el2 register
#[inline(always)]
pub fn read_spsr_el2() -> u64 {
    let spsr_el2: u64;
    unsafe {
        asm!("mrs {}, SPSR_EL2", out(reg) spsr_el2, options(nomem, nostack, preserves_flags));
    }
    spsr_el2
}

// read spsr_el1 register
#[inline(always)]
pub fn read_spsr_el1() -> u64 {
    let spsr_el1: u64;
    unsafe {
        asm!("mrs {}, SPSR_EL1", out(reg) spsr_el1, options(nomem, nostack, preserves_flags));
    }
    spsr_el1
}

#[inline(always)]
pub fn enable_hypervisor() {
    unsafe {
        asm!(
            "mov x0, xzr",
            "mov x0, #(1 << 31)",   // 64bit EL1
            "orr x0, x0, #(1 << 3)",  // Physical IRQ Routing ! MUST
            "orr x0, x0, #(1 << 4)",  // Physical Serror Routing 
            "orr x0, x0, #(1 << 5)",  // Physical Serror Routing
            "msr hcr_el2, x0",
            "ret",
            options(nostack)
        );
    }
}
