#![allow(unused)]
#![no_std]
#![no_main]

mod arch;
mod drivers;
mod bsp;

use drivers::gpio;
use core::panic::PanicInfo;
use crate::drivers::time;

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {

    panic!("hello");
    // gpio::setup(21, gpio::Modes::Output);
    // gpio::output(21, true);
    //
    // gpio::setup(2, gpio::Modes::Output);
    //
    loop {
        // gpio::output(2, false);
        // time::wait_msec(500_000);
        // gpio::output(2, true);
        // time::wait_msec(500_000);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Turn on the pin 21
    gpio::setup(21, gpio::Modes::Output);
    gpio::output(21, true);
    loop {}
}

