#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

use bootloader::entry_point;

entry_point!(traizzle_main);

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn traizzle_main(_info: &'static mut bootloader::BootInfo) -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}