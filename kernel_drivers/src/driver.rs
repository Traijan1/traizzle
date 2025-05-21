pub trait Writer {
    fn write(&mut self, buffer: &[u8]) -> usize;
}

pub trait Reader {
    fn read(&self, buffer: &mut [u8]) -> usize;
}

pub trait DriverLifecycle {
    fn init(&self);
    fn close(&self);
}

pub trait GraphicsWriter: Writer {
    fn clear_screen(&mut self);
    fn set_color(&mut self, color_code: u8);
    fn new_line(&mut self);
    fn scroll(&mut self);
    fn clear_row(&mut self, row: usize);
}
