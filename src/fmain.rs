use crate::desktop::{Desktop, DESKTOP_SIZE};
use crate::novusk::libs::libcolor::vga_colors::Color;

pub fn main_loop() {
    let mut desktop = unsafe { Desktop::new(Color::Cyan, DESKTOP_SIZE, true) };
    desktop.init();

    let mut exit = false;

    loop {
        if exit == true {
            break;
        }
    }
}
