#![no_std]
#![no_main]

#[macro_use] pub extern crate novusk;

#[cfg(target_arch = "x86_64")]
pub(crate) mod x86_64;

pub(crate) mod desktop;
pub(crate) mod fmain;

use fmain::main_loop;


#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    #[cfg(target_arch = "x86_64")]
    x86_64::x86_64_setup();

    main_loop();

    panic!("FOMOSv3-Blue v3 ended");
}
