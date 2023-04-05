//use volatile::Volatile;
use core::ptr::{self, read_volatile};

// Define GIC Distributor control register address
const GICD_CTLR: u64 = 0x0000_0100;

// Define GIC Distributor struct for accessing registers inside GICD
struct GicDistributor(u64);

impl GicDistributor {
    // Create a new instance of GIC Distributor struct
    fn new(base_addr: u64) -> Self {
        Self(base_addr)
    }

    // Initialize GIC Distributor
    fn init(&mut self) {
        // Set interrupt routing table
        // Set priority of interrupts
        // Other initialization operations
    }

    // Enable interrupt controller
    fn enable(&mut self) {
        unsafe {
            // On Aarch64 architecture, volatile pointers are used for MMIO operations
            let ctlr_ptr = (self.0 + GICD_CTLR) as *mut u32;
            let ctlr_value = ptr::read_volatile(ctlr_ptr);
            ptr::write_volatile(ctlr_ptr, ctlr_value | 1);
        }
    }

    // Disable interrupt controller
    fn disable(&mut self) {
        unsafe {
            // On Aarch64 architecture, volatile pointers are used for MMIO operations
            let ctlr_ptr = (self.0 + GICD_CTLR) as *mut u32;
            let ctlr_value = ptr::read_volatile(ctlr_ptr);
            ptr::write_volatile(ctlr_ptr, ctlr_value & !1);
        }
    }
}

// Define GIC CPU interface struct for accessing registers inside GICC
struct GicCpuInterface(u64);

impl GicCpuInterface {
    // Create a new instance of GIC CPU interface struct
    fn new(base_addr: u64) -> Self {
        Self(base_addr)
    }

    // Initialize GIC CPU interface
    fn init(&mut self) {
        // Initialization operations
    }

    // Enable interrupt
    fn enable_irq(&mut self, irq: u32) {
        unsafe {
            // Enable the specified IRQ
            let ptr = (self.0 + ((irq / 32) * 4) as u64) as *mut u32;
            let value = read_volatile(ptr);
            ptr::write_volatile(ptr, value | (1 << (irq % 32)));
        }
    }

    // Disable interrupt
    fn disable_irq(&mut self, irq: u32) {
        unsafe {
            // Disable the specified IRQ
            let ptr = (self.0 + ((irq / 32) * 4) as u64) as *mut u32;
            let value = read_volatile(ptr);
            ptr::write_volatile(ptr, value & !(1 << (irq % 32)));
        }
    }
}
