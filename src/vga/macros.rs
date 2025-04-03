use core::fmt;
use crate::vga::Color;
use lazy_static::lazy_static;
use spin::Mutex;
use super::Writer;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = {
        let writer = Writer::new();
        Mutex::new(writer)
    };
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::macros::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

pub fn set_color(foreground: Color, background: Color) {
    WRITER.lock().set_color(foreground, background);
}

pub fn clear_screen() {
    WRITER.lock().clear_terminal();
}