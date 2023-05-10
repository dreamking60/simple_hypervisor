use super::regs::Regs;
use super::asm::read_spsr_el1;
use super::asm::read_elr_el2;

#[derive(Copy, Clone)]
pub struct VCPU {
    vcpu_regs: Regs,
    spsr_el1: u64,
    elr_el2: u64,
}

// save vcpu
pub fn save_vcpu(vcpu: &mut VCPU, regs: &Regs) {
    vcpu.vcpu_regs = *regs;
    vcpu.elr_el2 = read_elr_el2();
    vcpu.spsr_el1 = read_spsr_el1();
}

// restore vcpu
pub fn restore_cpu(vcpu: &mut VCPU, regs: &mut Regs) {
    vcpu.vcpu_regs = *regs;
    vcpu.elr_el2 = read_elr_el2();
    vcpu.spsr_el1 = read_spsr_el1();
}

