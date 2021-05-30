mod lcdc;

pub trait Byte {
    fn read(&self) -> u8;
    fn write(&mut self, value: u8) -> u8;
}
