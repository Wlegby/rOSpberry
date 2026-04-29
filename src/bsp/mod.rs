#[cfg(all(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
compile_error!("Only one BSP feature can be enabled at a time.");

#[cfg(feature = "bsp_rpi3")]
pub mod r3bp;
#[cfg(feature = "bsp_rpi3")]
pub use r3bp::*;

#[cfg(feature = "bsp_rpi4")]
pub mod r4b;
#[cfg(feature = "bsp_rpi4")]
pub use r4b::*;
