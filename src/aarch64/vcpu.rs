use super::regs::Regs;

pub struct VCPU {
    vcpu_regs: Regs,
    spsr_el1: u64,
    elr_el2: u64,
    sp_el1: u64,
    pc: u64,
}