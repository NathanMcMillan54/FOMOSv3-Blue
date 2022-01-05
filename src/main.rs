#![no_std]
#![no_main]

#[macro_use] extern crate alloc;
#[macro_use] extern crate novuskinc;
#[macro_use] extern crate novusk;

use core::fmt::Write;
use novusk::libs::{libost::desktop::{Desktop, DesktopIcon}, libwin::Window, libwin::graphics::graphics::*};

pub mod builtins;
pub mod fk;
pub mod kernel;
pub mod setup;

pub use fk::*;

#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    setup::setup_fomos();
    fk_print!("Setup finished\n");

    panic!("FOMOSv3-Blue v3 ended");
}
