/// Represents a single port
#[derive(Clone, Copy, Debug)]
pub(super) struct Port {
    port: u16
}

impl Port {
    pub fn new(port: u16) -> Self {
        Self {
            port
        }
    }

    /// Write an u8 to the port
    #[inline(always)]
    pub fn write_u8(&self, data: u8) {
        commands::outb(self.port, data);
        Self::io_wait();
    }

    /// Write an u8 to the port but with an offset
    #[inline(always)]
    pub fn write_u8_offset(&self, offset: u16, data: u8) {
        commands::outb(self.port + offset, data);
        Self::io_wait();
    }

    /// Read value from port
    #[inline(always)]
    pub fn read_u8(&self) -> u8 {
        commands::inb(self.port)
    }

    /// Read value from port with an offset
    #[inline(always)]
    pub fn read_u8_offset(&self, offset: u16) -> u8 {
        commands::inb(self.port + offset)
    }
 
    /// Wait a single port write
    #[inline(always)]
    pub fn io_wait() {
        commands::outb(0x80, 0);
    }
}

pub(super) mod commands {
    use core::arch::asm;

    /// Write to specified port (asm instruction)
    #[inline(always)]
    pub fn outb(port: u16, data: u8) {
        unsafe {
            asm!(r#"
                    out dx, al
                "#,
                in("dx") port,
                in("al") data
            )
        }
    }

    /// Read from specified port (asm instruction)
    #[inline(always)]
    pub fn inb(port: u16) -> u8 {
        let mut data: u8;

        unsafe {
            asm!(r#"
                    in al, dx
                "#,
                in("dx") port,
                out("al") data
            )
        }

        data
    }
}