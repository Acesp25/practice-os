#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;
use vga::{Color, macros};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    macros::set_color(Color::Black, Color::LightGray);
    macros::clear_screen();
    
    let x: u8 = 10;
    print!("Hello huzz #{}", x);
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
