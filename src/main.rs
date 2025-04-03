#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;
use vga::{Writer, Color};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut writer = Writer::new();
    writer.set_color(Color::White, Color::Blue);
    let x: u8 = 10;
    print!("Hello huzz #{}", x);
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
