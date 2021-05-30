pub mod lcdc;
pub mod lcds;

pub trait Byte {
    fn read(&self) -> u8;
    fn write(&mut self, value: u8);
}
