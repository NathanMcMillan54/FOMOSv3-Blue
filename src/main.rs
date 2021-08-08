#![no_std]
#![no_main]

#[macro_use] pub extern crate novusk;

#[cfg(target_arch = "aarch64")]
pub(crate) mod aarch64;

#[cfg(target_arch = "x86_64")]
pub(crate) mod x86_64;

#[cfg(target_arch = "x86_64")]
pub(crate) mod desktop;
#[cfg(target_arch = "x86_64")]
pub(crate) mod fmain;


#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    #[cfg(target_arch = "aarch64")]
    aarch64::aarch64_setup();

    #[cfg(target_arch = "x86_64")]
    x86_64::x86_64_setup();

    #[cfg(target_arch = "x86_64")]
    fmain::main_loop();

    panic!("FOMOSv3-Blue v3 ended");
}
