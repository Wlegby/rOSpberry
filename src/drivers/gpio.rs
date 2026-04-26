use crate::bsp::memory::GPIO_BASE;
use core::ptr::{read_volatile, write_volatile};

const GPSET0: usize = GPIO_BASE + 0x1C;
const GPCLR0: usize = GPIO_BASE + 0x28;
const GPLEV0: usize = GPIO_BASE + 0x34;

pub enum Modes {
    Input,
    Output,
}

impl Modes {
    fn val(&self) -> u32 {
        match self {
            Self::Input => 0,
            Self::Output => 1,
        }
    }
}

pub fn setup(pin: usize, mode: Modes) {
    let offset: usize = match pin {
        0..=9 => 0,
        10..=19 => 1,
        20..=27 => 2,
        _ => 10,
    };

    if offset == 10 {
        panic!("Invalid pin");
    }

    unsafe {
        let dst = (GPIO_BASE + offset) as *mut u32;
        let curr_register = read_volatile(dst);

        write_volatile(dst, curr_register | (mode.val() << ((pin % 10) * 3)));
    }
}

pub fn output(pin: usize, high: bool) {
    let dst = if high { GPSET0 } else { GPCLR0 } as *mut u32;

    unsafe { write_volatile(dst, 1 << pin) };
}

pub fn read(pin: usize) -> bool {
    let register = unsafe { read_volatile(GPLEV0 as *mut u32) };
    (register >> pin) & 1 == 1
}

pub fn cleanup() {
    for i in 0..=27 {
        setup(i, Modes::Input);
    }
}
