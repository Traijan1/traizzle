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
use crate::driver::graphics::fonts::psf::PSF;

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

    let font: &[u8] = include_bytes!("zap-light16.psf");
    let psf = PSF::new(font);
    
    let mut framebuffer = Framebuffer::new(buffer, info.stride, info.bytes_per_pixel, psf);

    framebuffer.clear();

    // framebuffer.draw_rectangle(300, 300, 0x0000FF00, 0, 0);
    // framebuffer.draw_rectangle(100, 100, 0x02434633, 100, 350);
    // framebuffer.draw_rectangle(100, 100, 0x007F2F34, 0, 10);

    let mut i = 0;

    "Hello World".chars().for_each(|char| {
        framebuffer.draw_char(char, 8 * 4 * i, 0);
        i += 1;
    });

    loop {
        asm::hlt();
    }
}