pub struct Port {
    port: u16
}

impl Port {
    pub fn new(port: u16) -> Self {
        Self {
            port
        }
    }

    pub fn write_u8(&self, data: u8) {
        commands::outb(self.port, data);
        Self::io_wait();
    }

    pub fn write_u8_offset(&self, offset: u16, data: u8) {
        commands::outb(self.port + offset, data);
        Self::io_wait();
    }

    pub fn read_u8(&self) -> u8 {
        commands::inb(self.port)
    }

    pub fn read_u8_offset(&self, offset: u16) -> u8 {
        commands::inb(self.port + offset)
    }
 
    pub fn io_wait() {
        commands::outb(0x80, 0);
    }
}

pub mod commands {
    use core::arch::asm;

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