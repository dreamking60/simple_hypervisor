use super::asm::disable_cnthp_ctl_el2;
use super::asm::enable_cnthp_ctl_el2;
use super::asm::enable_irq;
use super::asm::read_cntfrq_el0;
use super::asm::read_cnthp_ctl_el2;
use super::asm::read_cntpct_el0;
use super::asm::write_cnthp_cval_el2;
use crate::println;

static mut CNTFRQ: u64 = 0;
static mut TIMER_WAIT: u64 = 2;

pub fn set_el2_timer_sec(sec: u64) {
    unsafe {
        TIMER_WAIT = sec;
    }
}

pub fn timer_handler() {
    let ticks: u64;
    let current_cnt: u64;
    let val: u64;

    // disable the timer
    disable_cnthp_ctl_el2();
    val = read_cnthp_ctl_el2();
    println!("REG(CNTHP_CTL_EL2): 0x{:0x}", val);

    unsafe {ticks = TIMER_WAIT * CNTFRQ;}

    current_cnt = read_cntpct_el0();
    write_cnthp_cval_el2(current_cnt + ticks);

    // enable the timer
    enable_cnthp_ctl_el2();
    enable_irq();

}

pub fn timer_el2_init() {
    // disable the timer
    disable_cnthp_ctl_el2();

    // read the current counter
    unsafe {
        CNTFRQ = read_cntfrq_el0();
    }

    // enable the timer
    enable_cnthp_ctl_el2();
    enable_irq();
}   