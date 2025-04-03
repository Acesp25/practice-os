use core::fmt;
use super::{Buffer, Char, Color};

pub struct Writer {
    row: usize,
    column: usize,
    color: u8,
    buffer: &'static mut Buffer,
}
impl Terminal {
    const WIDTH: usize = 80;
    const HEIGHT: usize = 25;

    pub fn new() -> Terminal {
        let buffer = unsafe { &mut *(0xb8000 as *mut Buffer) };
        Terminal {
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
        for row in 0..Self::HEIGHT {
            for col in 0..Self::WIDTH {
                self.buffer.chars[row][col] = Char {
                    character: b' ',
                    color: self.color,
                };
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
            for row in 0..Self::HEIGHT - 1{
                for col in 0..Self::WIDTH {
                    self.buffer.chars[row][col] = self.buffer.chars[row + 1][col];
                }
            }
            for col in 0..Self::WIDTH {
                self.buffer.chars[Self::HEIGHT - 1][col] = Char {
                    character: b' ',
                    color: self.color,
                };
            }
            self.row = Self::HEIGHT - 1;
            self.column = 0;
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.bound_check();

        if byte == b'\n' {
            self.column = 0;
            self.row += 1;
            return;
        }
        
        self.buffer.chars[self.row][self.column] = Char {
            character: byte,
            color: self.color,
        };
        
        self.column += 1;
    }

    pub fn write(&mut self, input: &[u8]) {
        for &byte in input {
            self.write_byte(byte);
        }
    }
}

impl fmt::Write for Terminal {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s.as_bytes());
        Ok(())
    }
}