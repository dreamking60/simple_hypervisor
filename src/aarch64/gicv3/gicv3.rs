// Purpose: GICv3 Driver
const GICD_BASE_ADDR: usize = 0x0800_0000;
const GICC_BASE_ADDR: usize = 0x0800_1000;

// GICD Control Register
const GICD_CTLR: *mut u32 = (GICD_BASE_ADDR + 0x0000) as *mut u32;
const GICD_ISENABLER: *mut u32 = (GICD_BASE_ADDR + 0x0100) as *mut u32;
const GICD_ICENABLER: *mut u32 = (GICD_BASE_ADDR + 0x0180) as *mut u32;
const GICD_ISPENDR: *mut u32 = (GICD_BASE_ADDR + 0x0200) as *mut u32;
const GICD_ICPENDR: *mut u32 = (GICD_BASE_ADDR + 0x0280) as *mut u32;
const GICD_IPRIORITYR: *mut u32 = (GICD_BASE_ADDR + 0x0400) as *mut u32;
const GICD_ITARGETSR: *mut u32 = (GICD_BASE_ADDR + 0x0800) as *mut u32;
const GICD_ICFGR: *mut u32 = (GICD_BASE_ADDR + 0x0C00) as *mut u32;
const GICD_SGIR: *mut u32 = (GICD_BASE_ADDR + 0x0F00) as *mut u32;

// GICC Control Register
const GICC_CTLR: *mut u32 = (GICC_BASE_ADDR + 0x0000) as *mut u32;
const GICC_PMR: *mut u32 = (GICC_BASE_ADDR + 0x0004) as *mut u32;
const GICC_IAR: *mut u32 = (GICC_BASE_ADDR + 0x000C) as *mut u32;
const GICC_EOIR: *mut u32 = (GICC_BASE_ADDR + 0x0010) as *mut u32;

// GIC CPU Interface
const GICC_CTLR_ENABLE: u32 = 0x1;
const GICC_CTLR_FIQEN: u32 = 0x2;
const GICC_CTLR_CBPR: u32 = 0x4;
const GICC_CTLR_EOImodeNS: u32 = 0x8;
const GICC_CTLR_EOImodeS: u32 = 0x10;
const GICC_CTLR_EOImode: u32 = 0x18;
const GICC_CTLR_BPR: u32 = 0x7 << 3;
const GICC_CTLR_PRIOR: u32 = 0xFF << 23;

// GIC Distributor
const GICD_CTLR_ENABLE: u32 = 0x1;
const GICD_CTLR_ARE_NS: u32 = 0x2;
const GICD_CTLR_ARE_S: u32 = 0x4;
const GICD_CTLR_ARE: u32 = 0x6;
const GICD_CTLR_DS: u32 = 0x8;
const GICD_CTLR_E1NWF: u32 = 0x100;
const GICD_CTLR_E1NS: u32 = 0x200;
const GICD_CTLR_E1S: u32 = 0x400;
const GICD_CTLR_E1: u32 = 0x600;
const GICD_CTLR_NLBPR: u32 = 0x7 << 5;
const GICD_CTLR_RWP: u32 = 0x1 << 7;

// GIC Interrupt Group
const GICD_ICFGR_LEVEL: u32 = 0x0;
const GICD_ICFGR_EDGE: u32 = 0x2;
const GICD_ICFGR_MASK: u32 = 0x2;

// GIC Interrupt Priority
const GICD_IPRIORITYR_PRIORITY: u32 = 0xFF;

