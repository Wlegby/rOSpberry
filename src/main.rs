#![allow(unused)]
#![no_std]
#![no_main]

mod arch;
mod drivers;
mod bsp;
mod dbg;

use heapless::format;
use heapless::String;
use heapless::string::StringInner;

use core::ptr::addr_of_mut;
use core::panic::PanicInfo;

use drivers::gpio;
use drivers::time;
use drivers::console::Console;
use drivers::framebuffer::FrameBuffer;

static LOGO: &[u8] = include_bytes!("logo.raw");

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {

    panic!();

    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Turn on the pin 21
    gpio::setup(21, gpio::Modes::Output);
    gpio::output(21, true);

    let mut framebuff = FrameBuffer::new(1024, 768, 32); 

    let response = framebuff.init();

    if let Err(_) = response {
        panic!();
    }

    let c = Console::init(framebuff);
    let mut console = match c {
        Some(c) => c,
        None => {
            dbg::fail();
            panic!();
        }
    };

    let mut text = StringInner::new();
    text.push_str(_info.message().as_str().unwrap());
    text.push('\n');
    text.push_str(_info.location().unwrap().file());

    console.print(text);

    loop {}
}

