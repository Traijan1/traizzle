use super::graphics::{framebuffer::{Framebuffer, self}, fonts::psf::PSF};

use core::fmt;

macro_rules! mut_framebuffer {
    ($framebuffer:ident) => {
        $framebuffer.framebuffer.as_mut().expect("Cannot borrow framebuffer as mutable")
    };
}

pub struct Console<'a> {
    framebuffer: &'a mut Option<Framebuffer<'a>>,
    max_column: usize,
    column: usize,
    row: usize
}

impl<'a> Console<'a> {
    pub fn new(framebuffer: &'a mut Option<Framebuffer<'a>>) -> Self {
        let cache_buffer = framebuffer.as_mut().unwrap();
        let max_column = cache_buffer.stride / PSF::CHAR_WIDTH;

        Self { 
            framebuffer,
            max_column,
            column: 0,
            row: 0
        }
    }

    pub fn write(&mut self, text: &str) {
        for char in text.chars() {
            if self.handle_special_character(char) {
                continue;
            }
            
            let frame = mut_framebuffer!(self);
            
            frame.print_char(char, self.column * 8 * frame.channels, self.row);
            self.column += 1;

            if self.column >= self.max_column {
                self.handle_special_character('\n');
            }
        }
    }

    pub fn clear(&mut self) {
        self.framebuffer.as_mut().unwrap().clear();
        self.column = 0;
        self.row = 0;
    }

    #[inline(always)]
    pub fn change_foreground_color(&mut self, color: u32) {
        let framebuffer = mut_framebuffer!(self);
        framebuffer.foreground = color;
    }

    #[inline(always)]
    pub fn change_background_color(&mut self, color: u32) {
        let framebuffer = mut_framebuffer!(self);
        framebuffer.background = color;
    }

    #[inline(always)]
    pub fn change_color(&mut self, foreground: u32, background: u32) {
        self.change_foreground_color(foreground);
        self.change_background_color(background);
    }

    #[inline(always)]
    fn handle_special_character(&mut self, c: char) -> bool {
        match c {
            '\n' => {
                self.row += 1;
                self.column = 0;

                true
            },
            '\t' => {
                self.write("    "); // 1 Tab = 4 Spaces
                true
            },
            _ => false
        }
    }
}

impl<'a> fmt::Write for Console<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);        
        Ok(())
    }
}