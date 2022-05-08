#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

mod driver;
mod utils;

use utils::debug;
use utils::asm;

entry_point!(traizzle_main);

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

/// The entry point for **Traizzle**
#[no_mangle]
fn traizzle_main(_info: &'static mut BootInfo) -> ! {
    log!("Hello World from Kernel to Serial Port!");
    log!("Test");
    
    loop {
        asm::hlt();
    }
}