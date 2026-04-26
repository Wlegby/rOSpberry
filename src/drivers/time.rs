use crate::bsp::memory::*;
use core::{arch::asm, ptr::read_volatile};

const CS: usize = TIMER_BASE;
const CLO: usize = CS + 0x4;
const CHI: usize = CLO + 0x4;
const C0: usize = CHI + 0x4;
const C1: usize = C0 + 0x4;
const C2: usize = C1 + 0x4;
const C3: usize = C2 + 0x4;

pub fn get_system_timer() -> u64 {
    let (h, l) = unsafe {
        (
            read_volatile(CLO as *const u32) as u64,
            read_volatile(CHI as *const u32) as u64,
        )
    };
    return (h << 32) | l;
}

pub fn wait_msec(n: u64) {
    let t = get_system_timer();

    if t == 0 {
        panic!("system time not working");
    }

    while get_system_timer() - t < n {
        unsafe { asm!("nop") }
    }
}
