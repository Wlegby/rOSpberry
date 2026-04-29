#[cfg(feature = "bsp_rpi3")]
pub mod r3bp;
#[cfg(feature = "bsp_rpi3")]
pub use r3bp::*;

#[cfg(feature = "bsp_rpi4")]
pub mod r4b;
#[cfg(feature = "bsp_rpi4")]
pub use r4b::*;
