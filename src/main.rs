#![no_std]
#![no_main]

pub use novusk;

#[cfg(target_arch = "x86_64")]
pub(crate) mod x86_64;


pub(crate) mod graphics;

#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    #[cfg(target_arch = "x86_64")]
    x86_64::x86_64_setup();

    loop {  }
}
