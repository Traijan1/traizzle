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

    let console: Console;

    unsafe {
        console = Console::<'static>::new(&mut FRAMEBUFFER);
    }

    unsafe {
        let _ = CONSOLE.insert(console);
    }

    println!("Hello World {}", 10);

    loop {
        asm::hlt();
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
fn _print(args: core::fmt::Arguments) {
    unsafe {
        let _ = CONSOLE.as_mut().unwrap().write_fmt(args);
    }
}