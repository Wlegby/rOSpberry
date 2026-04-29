fn main() {
    // Tell Cargo to re-run if the features change
    println!("cargo:rerun-if-changed=src/bsp/r3bp/linker.ld");
    println!("cargo:rerun-if-changed=src/bsp/r4b/linker.ld");

    if cfg!(feature = "bsp_rpi3") {
        println!("cargo:rustc-link-arg=-T./src/bsp/r3bp/linker.ld");
    } else if cfg!(feature = "bsp_rpi4") {
        println!("cargo:rustc-link-arg=-T./src/bsp/r4b/linker.ld");
    }
}
