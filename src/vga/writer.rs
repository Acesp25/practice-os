use core::fmt;
use super::{Buffer, Color};
use crate::vga::buffer::Char;

pub struct Writer {
    row: usize,
    column: usize,
    color: u8,
    buffer: &'static mut Buffer,
}

impl Writer {
    const WIDTH: usize = 80;
    const HEIGHT: usize = 25;

    pub fn new() -> Writer {
        let buffer = unsafe { &mut *(0xb8000 as *mut Buffer) };
        Writer {
            row: 0,
            column: 0,
            color: 0x07,
            buffer,
        }
    }

    pub fn set_color(&mut self, foreground: Color, background: Color) {
        self.color = ((background as u8) << 4) | (foreground as u8);
    }

    pub fn clear_terminal(&mut self) {
        let blank = Char {
            character: b' ',
            color: self.color,
        };
        for row in 0..Self::HEIGHT {
            for col in 0..Self::WIDTH {
                self.buffer.write_char(row, col, blank);
            }
        }
        self.row = 0;
        self.column = 0;
    }

    fn bound_check(&mut self) {
        if self.column >= Self::WIDTH {
            self.column = 0;
            self.row += 1;
        }

        if self.row >= Self::HEIGHT {
            for row in 1..Self::HEIGHT {
                for col in 0..Self::WIDTH {
                    let character = self.buffer.read_char(row, col);
                    self.buffer.write_char(row - 1, col, character);
                }
            }
            let blank = Char {
                character: b' ',
                color: self.color,
            };
            for col in 0..Self::WIDTH {
                self.buffer.write_char(Self::HEIGHT - 1, col, blank);
            }
            self.row = Self::HEIGHT - 1;
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                self.column = 0;
                self.row += 1;
                self.bound_check();
            }
            byte => {
                if self.column >= Self::WIDTH {
                    self.column = 0;
                    self.row += 1;
                    self.bound_check();
                }
                let char = Char {
                    character: byte,
                    color: self.color,
                };
                self.buffer.write_char(self.row, self.column, char);
                self.column += 1;
            }
        }
    }

    pub fn write(&mut self, input: &[u8]) {
        for &byte in input {
            self.write_byte(byte);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s.as_bytes());
        Ok(())
    }
}