#![no_std]
#![no_main]

extern crate novusk;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    loop {  }
}
