use novusk::libs::libcolor::vga_colors::Color;

pub trait Drawer {
    fn pixel(&mut self, color: Color, position: usize) {

    }

    fn line(&mut self, color: Color, position: usize) {
        for i in 0..position {
            self.pixel(color, i);
        }
    }
}