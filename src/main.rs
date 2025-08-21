#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[allow(unreachable_code)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    panic!("OH NO");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
