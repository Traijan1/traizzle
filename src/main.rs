#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;
use core::format_args;

use bootloader::{entry_point, BootInfo};

mod driver;
mod utils;
mod asm;

use utils::debug;

use driver::graphics::framebuffer::Framebuffer;
use driver::console::Console;
use driver::graphics::fonts::psf::PSF;

entry_point!(traizzle_main);

static mut CONSOLE: Option<Console> = None;
static mut FRAMEBUFFER: Option<Framebuffer> = None;

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

    let font: &[u8] = include_bytes!("assets/zap-light16.psf");
    let psf = PSF::new(font);
    
    let framebuffer = Framebuffer::<'static>::new(buffer, info.stride, info.bytes_per_pixel, psf);

    unsafe {
        let _ = FRAMEBUFFER.insert(framebuffer);
    }

    unsafe {
        FRAMEBUFFER.as_mut().unwrap().clear();
    }

    // framebuffer.draw_rectangle(300, 300, 0x0000FF00, 0, 0);
    // framebuffer.draw_rectangle(100, 100, 0x02434633, 100, 350);
    // framebuffer.draw_rectangle(100, 100, 0x007F2F34, 0, 10);

    unsafe {
        FRAMEBUFFER.as_mut().unwrap().print_char('C',0, 0);
    }

    let mut console: Console;

    unsafe {
        console = Console::<'static>::new(&mut FRAMEBUFFER);
    }

    unsafe {
        let _ = CONSOLE.insert(console);
    }

    unsafe {
        CONSOLE.as_mut().unwrap().write("Hello World! ");
    }

    test();

    unsafe {
        CONSOLE.as_mut().unwrap().write_fmt(format_args!("Number: {}", 10));
    }

    loop {
        asm::hlt();
    }
}

fn test() {
    unsafe {
        CONSOLE.as_mut().unwrap().write("Hello World! ");
    }
}