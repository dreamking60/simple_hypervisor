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

// currentEL
pub fn get_current_el() -> u32 {
    let current_el: u64;
    unsafe {
        asm!("mrs {}, CurrentEL", out(reg) current_el, options(nomem, nostack, preserves_flags));
    }
    (current_el >> 2) as u32
}