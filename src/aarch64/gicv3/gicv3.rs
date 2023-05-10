// Purpose: GICv3 Driver
// GICD Base Addr
const GICD_BASE_ADDR: usize = 0x0800_0000;

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


