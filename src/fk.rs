use core::fmt::{Arguments, Write};

struct FkWriter {
    pub x: usize,
    pub y: usize,
}

impl FkWriter {
    fn new() -> Self {
        return FkWriter {
            x: 0,
            y: 0,
        };
    }

    fn write(&self, write: &str) {

    }
}

impl Write for FkWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s);

        Ok(())
    }
}
