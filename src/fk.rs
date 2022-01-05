use core::borrow::Borrow;
use core::fmt::{Arguments, Write};
use core::str::from_utf8_unchecked;
use crate::graphics_write;

pub static mut FKWRITER: FkWriter = FkWriter { x: 1, y: 0 };

pub struct FkWriter {
    pub x: usize,
    pub y: usize,
}

impl FkWriter {
    pub fn new() -> Self {
        return FkWriter {
            x: 1,
            y: 0,
        };
    }

    pub fn write(&mut self, write: u8) {
        if write == b'\n' {
            self.y += 9;
            self.x = 1;
        } else { self.x += 8; }

        unsafe { graphics_write(self.x, self.y, 15, from_utf8_unchecked(&[write])); }
    }
}

impl Write for FkWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.as_bytes() {
            self.write(*b);
        }

        Ok(())
    }
}

pub fn _fk_print(fmt: Arguments) -> Arguments {
    unsafe { FKWRITER.write_fmt(fmt); }

    return fmt;
}

#[macro_export]
macro_rules! fk_print {
    ($($arg:tt)*) => {$crate::fk::_fk_print(format_args!($($arg)*))};
}
