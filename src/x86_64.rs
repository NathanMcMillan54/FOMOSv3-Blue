use crate::graphics::drawer::Drawer;
use novusk::libs::libcolor::vga_colors::Color;
use novusk::x86::drivers::vga::pixel::_pixel;
use novusk::x86::drivers::vga::vga_80x25::*;
use novusk::x86::drivers::vga::VGA_ADDRESS;
use novusk::kernel::printk::printk;
use core::fmt::Write;

pub(crate) fn x86_64_setup() {
    let mut drawer = X64Drawer;

    // Clear the screen
    for i in 0..25 {
        for i in 0..81 {
            drawer.line(Color::Cyan, i);
        }

        if i != 24 {
            unsafe { printk!(""); }
        }
    }

    let mut writer = Vga80x25 {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Cyan),
        buffer: unsafe { &mut *(VGA_ADDRESS as *mut Buffer ) }
    };


    writer.write_fmt(format_args!("{}", "Starting FOMOSv3-Blue v3...\n"));
}

struct X64Drawer;

impl Drawer for X64Drawer {
    fn pixel(&mut self, color: Color, position: usize) {
        _pixel(color, position);
    }
}
