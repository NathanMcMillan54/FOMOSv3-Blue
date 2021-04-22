#![no_std]
#![no_main]

extern crate novusk;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    loop {  }
}

#[no_mangle]
pub extern "C" fn main_test() -> i32 {
    0
}