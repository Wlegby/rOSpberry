#![no_std]
#![no_main]

mod arch;
mod drivers;
mod bsp;
mod dbg;

extern crate alloc;
use linked_list_allocator::LockedHeap;
use core::ptr::addr_of_mut;
use alloc::format;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();
const HEAP_SIZE: usize = 100* 1024; // 100KB
                                    //
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

pub fn init_heap() {
    dbg::fail();
    unsafe {
        let heap_start = addr_of_mut!(HEAP) as *mut u8;
        ALLOCATOR.lock().init(heap_start, HEAP_SIZE);
    }
    dbg::success();
}

use drivers::gpio;
use drivers::time;
use drivers::console;
use core::panic::PanicInfo;
use crate::drivers::console::Console;
use crate::drivers::gpio::setup;
use crate::{drivers::framebuffer::FrameBuffer};

static LOGO: &[u8] = include_bytes!("logo.raw");

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {

    // We still need not make a MMU (Memory management unit)
    // init_heap();
    // let mut framebuff = FrameBuffer::new(1024, 768, 32); 
    //
    // let response = framebuff.init();
    //
    // if let Err(_) = response {
    //     panic!();
    // }
    //
    // framebuff.draw_image(512, 512, LOGO);


    // let c = Console::init(framebuff);
    // let mut console = match c {
    //     Some(c) => c,
    //     None => {
    //         dbg::fail();
    //         panic!();
    //     }
    // };
    //
    // for i in 0..100 {
    //     console.print(&format!("line: {}", i));
    //     time::wait_millis(500);
    // }

    gpio::setup(16, gpio::Modes::Output);

    loop {
        gpio::output(16, true);
        time::wait_millis(500);
        gpio::output(16, false);
        time::wait_millis(500);
    }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Turn on the pin 21
    gpio::setup(21, gpio::Modes::Output);
    gpio::output(21, true);
    loop {}
}

