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
    
    for x in 1..40 {
        terminal.write_byte(b'a' + x as u8 );
        terminal.write_byte(b'\n');
    }
    loop {}
}
