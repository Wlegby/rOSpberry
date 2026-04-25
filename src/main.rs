#![allow(unused)]
#![no_std]
#![no_main]

mod arch;
mod drivers;
mod bsp;

use drivers::gpio;
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Turn on the pin 21
    gpio::setup(21, gpio::Modes::Output);
    gpio::output(21, true);
    loop {}
}

