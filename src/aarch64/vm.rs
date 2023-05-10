use super::vcpu::{VCPU};

#[derive(Clone, Copy)]
pub struct VM {
    vcpu: VCPU,
    vmid: u32,
    memory_region: u64,
    entry_point: u64,
}