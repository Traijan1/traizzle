#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

mod driver;
mod utils;

use driver::serial_port;

entry_point!(traizzle_main);

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn traizzle_main(_info: &'static mut BootInfo) -> ! {
    let test = serial_port::initialize_serial();

    log!("Hello World from Kernel to Serial Port!");
    log!("Test");

    loop {
        unsafe { asm!("hlt"); }
    }
}