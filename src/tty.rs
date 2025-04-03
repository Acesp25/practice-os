enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    Yellow = 14,
    White = 15
}

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
            color: 0x70,
            buffer,
        }
    }
    
    pub fn set_color(&mut self, foreground: Color, background: Color) {
        self.color = ((background as u8) << 4) | (foreground as u8);
    }

    pub fn write_byte(&mut self, byte: u8) {
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
