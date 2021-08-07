use novusk::libs::libcolor::vga_colors::Color;

extern "C" {
    fn _vga_pixel(color: Color, x: usize, y: usize);
}

pub fn x86_64_setup() {
    unsafe {
        _vga_pixel(Color::Cyan, 1, 1);
    }
}
