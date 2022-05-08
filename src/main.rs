#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

mod driver;
mod utils;
mod asm;

use utils::debug;

entry_point!(traizzle_main);

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    log!("Panic");
    loop {}
}

/// The entry point for **Traizzle**
#[no_mangle]
fn traizzle_main(_info: &'static mut BootInfo) -> ! {
    log!("Physical Memory Offset: {:#x}", _info.physical_memory_offset.as_ref().unwrap());

    loop {
        asm::hlt();
    }
}