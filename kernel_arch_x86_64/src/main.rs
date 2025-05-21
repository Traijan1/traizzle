#![no_std]
#![no_main]

use core::panic::PanicInfo;
use kernel_drivers::{driver::Writer, graphics::vga::VgaBuffer};

static HELLO: &[u8] = b"Hello World\n";

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut vga: VgaBuffer = VgaBuffer::new();
    vga.write(HELLO);

    loop {}
}