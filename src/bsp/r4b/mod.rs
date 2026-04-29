pub mod memory;

pub use crate::bsp::memory::*;

pub fn bus_to_phys(bus_address: u32) -> u32 {
    bus_address & !0xC000_0000 // Safer mask for Pi 3 alias
}
