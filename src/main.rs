#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

mod driver;
mod utils;
mod asm;

use utils::debug;

use driver::graphics::framebuffer::Framebuffer;

use crate::driver::graphics::framebuffer;

entry_point!(traizzle_main);

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    log!("Panic: {}", _info);
    loop {}
}

/// The entry point for **Traizzle**
#[no_mangle]
fn traizzle_main(_info: &'static mut BootInfo) -> ! {
    log!("Bootloader Version: {}.{}.{}", _info.version_major, _info.version_minor, _info.version_patch);
    log!("Physical Memory Offset: {:#x}", _info.physical_memory_offset.as_ref().unwrap());

    let info = &_info.framebuffer.as_ref().unwrap().info();
    let buffer = _info.framebuffer.as_mut().unwrap().buffer_mut();

    let mut framebuffer = Framebuffer::new(buffer, info.stride);

    framebuffer.draw_rectangle(300, 300, 0x0000FF00, 0, 0);
    framebuffer.draw_rectangle(100, 100, 0x02434633, 100, 350);
    framebuffer.draw_rectangle(100, 100, 0x007F2F34, 0, 10);

    let test = buffer[0] as *mut u8;

    loop {
        asm::hlt();
    }
}