use crate::graphics_pixel;

pub static mut MOUSEDEVICE: MouseDevice = MouseDevice::new();

pub struct MouseDevice {
    pub x_pos: usize,
    pub y_pos: usize,
}

impl MouseDevice {
    pub const fn new() -> Self {
        return MouseDevice { x_pos: 50, y_pos: 50 };
    }

    pub fn update(&mut self, nx: usize, ny: usize) {
        self.x_pos = nx;
        self.y_pos = ny;
    }

    pub fn display(&self) {
        for h in 0..4 {
            for w in 0..4 {
                graphics_pixel(self.x_pos + w, self.y_pos + h, 15);
            }
        }
    }
}
