#![no_std]
#![no_main]

pub use novusk;

#[cfg(target_arch = "x86_64")]
pub(crate) mod x86_64;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    #[cfg(target_arch = "x86_64")]
    x86_64::x86_64_setup();

    panic!("FOMOSv3-Blue v3 finished all processes");
}
