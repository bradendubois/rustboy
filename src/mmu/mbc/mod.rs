pub mod mbc0;

/// The traits that any memory bank controller (MBC) must implement
pub trait MBC {
    fn read_ram(&self, address: u16) -> u8;
    fn read_rom(&self, address: u16) -> u8;
    fn write_ram(&mut self, value: u8, address: u16);
    fn write_rom(&mut self, value: u8, address: u16);
}
