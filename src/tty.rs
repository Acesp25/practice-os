#[repr(transparent)]
pub struct Buffer {
    chars: [[Char; Buffer::WIDTH]; Buffer::HEIGHT]
}

impl Buffer {
    pub const WIDTH: usize = 80;
    pub const HEIGHT: usize = 25;
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Char {
    character: u8,
    color: u8,
}

pub struct Terminal {
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

    pub fn write(&mut self, input: &[u8]) {
        for &byte in input {
            if self.column >= Self::WIDTH {
                self.column = 0;
                self.row += 1;
            }
            if self.row >= Self::HEIGHT {
                for row in 1..Self::HEIGHT{
                    for col in 0..Self::WIDTH {
                        self.buffer.chars[row][col] = self.buffer.chars[row + 1][col];
                    }
                }
            }

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
    }
}
