use super::colors::Color;

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
pub struct Char {
    character: u8,
    color: u8,
}