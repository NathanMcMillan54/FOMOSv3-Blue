#![no_std]
#![no_main]

#[macro_use] extern crate alloc;
#[macro_use] extern crate novuskinc;
#[macro_use] extern crate novusk;

use core::fmt::Write;
use novusk::libs::{libost::desktop::{Desktop, DesktopIcon}, libwin::Window, libwin::graphics::graphics::*};

pub mod arch;
pub mod builtins;
pub mod fk;
pub mod fomos;
pub mod input;
pub mod kernel;
pub mod setup;

pub use fk::*;

#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    setup::setup_fomos();
    fk_print!("Setup finished\n");

    fk_print!("Starting main loop...\n");
    fomos::main_loop();

    panic!("FOMOSv3-Blue v3 ended");
}
