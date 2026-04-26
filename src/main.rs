#![no_std]
#![no_main]

mod arch;
mod drivers;
mod bsp;
mod dbg;

use drivers::gpio;
use core::panic::PanicInfo;
use crate::{dbg::{fail, success}, drivers::{framebuffer::{FrameBufferSettings, draw_image, init_framebuffer}}};

static LOGO: &[u8] = include_bytes!("logo.raw");

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    let mut settings = FrameBufferSettings {
        width: 1024,
        height: 768,
        depth: 32,
        ..Default::default()
    };

    let response = init_framebuffer(&mut settings);

    if let Ok(()) = response {
        success();
        draw_image(&mut settings, 512, 512, LOGO);
    } else {
        fail();
    }

    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Turn on the pin 21
    gpio::setup(21, gpio::Modes::Output);
    gpio::output(21, true);
    loop {}
}

