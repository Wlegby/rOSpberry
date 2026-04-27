#![no_std]
#![no_main]

mod arch;
mod drivers;
mod bsp;
mod dbg;

use drivers::gpio;
use drivers::time;
use drivers::font;
use core::panic::PanicInfo;
use crate::{drivers::framebuffer::FrameBuffer};

static LOGO: &[u8] = include_bytes!("logo.raw");

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    let mut framebuff = FrameBuffer::new(1024, 768, 32); 

    let response = framebuff.init();

    if let Err(_) = response {
        dbg::fail();
        panic!();
    }

    dbg::success();

    time::wait_sec(2);
    framebuff.draw_image( 512, 512, LOGO);
    time::wait_millis(10);
    framebuff.clear();
    font::draw_string(&framebuff, "Hello, world!", 0, 0, 0xFFFFFFFF);


    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Turn on the pin 21
    gpio::setup(21, gpio::Modes::Output);
    gpio::output(21, true);
    loop {}
}

