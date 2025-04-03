#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod tty;
use tty::Terminal;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut terminal = Terminal::new();
    
    terminal.write(b"Farts!");

    loop {}
}
