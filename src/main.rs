#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

mod boot {
    core::arch::global_asm!(
        ".section .text._start",
        ".global _start",
        "_start:",
            "mrs     x0, mpidr_el1",        // Read CPU ID
            "and     x0, x0, #3",           // Mask out the core ID
            "cbz     x0, 2f",               // If core ID is 0, continue to '2'
        "1:", 
            "wfe",                          // Otherwise, put cores 1, 2, 3 to sleep
            "b       1b",
        "2:",
            "mov     sp, #0x80000",         // Set stack pointer to start at 0x80000
            "b       kmain"                 // Jump to our Rust function
    );
}


#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    // Now you can safely use the stack and call functions!
    unsafe {
        let gpio_base = 0x3F20_0000 as *mut u32;
        
        // setup pin 21 to be an output & 20 to be an input
        core::ptr::write_volatile(gpio_base.add(2), (1 << 3) | 0);

        let mut on = false;
        let mut pressed = false;

        loop {
            let button = core::ptr::read_volatile(gpio_base.add(13));

            if button & (1<<20) != 0 && !pressed{
                on = !on;
                pressed = true;
                for _ in 0..10_000 { core::arch::asm!("nop"); }
            } 

            if button & (1<<20) == 0 {
                pressed = false;
                for _ in 0..10_000 { core::arch::asm!("nop"); }
            }


            if on {
                core::ptr::write_volatile(gpio_base.add(7), 1<<21);
            } else {
                core::ptr::write_volatile(gpio_base.add(10), 1<<21);
            }
        }

    }

}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

