#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

mod driver;
mod utils;

use driver::io::serial::Serial;

entry_point!(traizzle_main);

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn traizzle_main(_info: &'static mut BootInfo) -> ! {
    let serial = Serial::new(0x3F8);
    serial.write_str("Hello World!");

    // log!("Hello World from Kernel to Serial Port!");
    // log!("Test");

    loop {
        unsafe { asm!("hlt"); }
    }
}