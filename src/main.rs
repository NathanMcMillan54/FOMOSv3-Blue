#![no_std]
#![no_main]

#[cfg(target_arch = "x86_64")]
#[macro_use] extern crate alloc;

#[macro_use] pub(crate) extern crate novusk;
use novusk::libs::{libost::desktop::{Desktop, DesktopIcon}, libwin::Window, libwin::graphics::graphics::*};

pub(crate) mod builtins;
pub mod display;
pub mod kernel;

#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    let mut desktop = Desktop::new((640, 480), CYAN, Some(vec![DesktopIcon::new((0, 0), 0)]));

    desktop.init();
    builtins::terminal::terminal_main();

    panic!("FOMOSv3-Blue v3 ended");
}
