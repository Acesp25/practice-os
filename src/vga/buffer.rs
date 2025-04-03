#[repr(transparent)]
pub struct Buffer {
    chars: [[Char; Buffer::WIDTH]; Buffer::HEIGHT]
}

impl Buffer {
    pub const WIDTH: usize = 80;
    pub const HEIGHT: usize = 25;

    pub fn write_char(&mut self, row: usize, col: usize, char: Char) {
        self.chars[row][col] = char;
    }

    pub fn read_char(&self, row: usize, col: usize) -> Char {
        self.chars[row][col]
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Char {
    pub character: u8,
    pub color: u8,
}