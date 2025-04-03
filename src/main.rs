#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod tty;
use tty::{Terminal, Color::*};
use core::fmt::{Write};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut terminal = Terminal::new();
    
    terminal.set_color(Black, LightGray);
    write!(terminal, "Hello, world!").unwrap();

    loop {}
}
