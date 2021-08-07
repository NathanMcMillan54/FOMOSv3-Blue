use novusk::libs::libcolor::vga_colors::Color;
use novusk::x86_64::pixel::_vga_pixel;

pub static mut DESKTOP_SIZE: (usize, usize) = (0, 0);

pub struct Desktop {
    pub color: Color,
    pub dims: (usize, usize),
    pub cursor: bool,
    pub cursor_x: usize,
    pub cursor_y: usize,
}

impl Desktop {
    pub fn new(background_color: Color, desktop_size: (usize, usize), add_cursor: bool) -> Self {
        return Desktop {
            color: background_color,
            dims: desktop_size,
            cursor: add_cursor,
            cursor_x: 20,
            cursor_y: 20
        }
    }
    
    fn fill(&mut self) {
        let (sx, sy) = self.dims;

        for y in 0..sy {
            for x in 0..sx {
                _vga_pixel(self.color, x, y);
            }
        }
    }

    fn draw_cursor(&mut self) {
        _vga_pixel(Color::White, self.cursor_x, self.cursor_y);
    }

    pub fn init(&mut self) {
        self.fill();

        if self.cursor {
            self.draw_cursor();
        }
    }
}
