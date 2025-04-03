#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod tty;
use tty::Terminal;

static HELLO: &[u8] = b"Hello World!";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut terminal = Terminal::new();
    
    terminal.write(HELLO);
    terminal.write(b"\n");
    terminal.write(b"Welcome to the kernel!\n");

    loop {}
}
