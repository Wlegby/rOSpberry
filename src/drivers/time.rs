use crate::bsp::memory::*;
use core::ptr::read_volatile;

const CS: usize = MMIO_BASE + 0x3000;
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
    return h | (l << 32);
}

pub fn wait_sec(n: u64) {
    wait_millis(n * 1000);
}

pub fn wait_millis(n: u64) {
    wait_microsec(n * 1000);
}

pub fn wait_microsec(n: u64) {
    let t = get_system_timer();

    if t == 0 {
        panic!("system time not working");
    }

    while get_system_timer() - t < n {
        // makes the cpu optimize
        core::hint::spin_loop();
    }
}
