#![no_std]
#![no_main]

pub use novusk;
use novusk::kernel::printk::printk;


#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    printk!("Starting FOMOSv3-Blue...");
    loop {  }
}
