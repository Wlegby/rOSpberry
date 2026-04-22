#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

// 0x 3E20 0008 1<<3
// 0x 3E1C 001C 1<<21
// 0x 3E28 001C 1<<21

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
        
        // GPFSEL2 is offset 0x08. Set Pin 21 to Output (001 in bits 3-5)
        core::ptr::write_volatile(gpio_base.add(2), 1 << 3);

        loop {
            // Turn pin 21 ON (GPSET0 is offset 0x1C / 4 = 7)
            core::ptr::write_volatile(gpio_base.add(7), 1 << 21);

            for _ in 0..100_000 { asm!("nop"); }

            // Turn pin 21 OFF (GPCLR0 is offset 0x28 / 4 = 10)
            core::ptr::write_volatile(gpio_base.add(10), 1 << 21);

            for _ in 0..100_000 { asm!("nop"); }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

