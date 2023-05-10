use super::vcpu::VCPU;

pub struct VM {
    vcpu: VCPU,
    vmid: u32,
    memory_region: u64,
    entry_point: u64,
}