#![no_std]
#![no_main]

#[macro_use] extern crate lazy_static;
#[macro_use] pub extern crate novusk;
use novusk::drivers::gpu::GpuGraphics;

pub mod display;
pub mod kernel;

#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    let mut gg = GpuGraphics::new();

    gg.graphics_print(200, 260, 15, format_args!("Setting up desktop..."));

    panic!("FOMOSv3-Blue v3 ended");
}
