pub mod mbc0;
pub mod mbc1;

/// The traits that any memory bank controller (MBC) must implement
pub trait MBC {
    fn read_ram(&mut self, address: u16) -> u8;
    fn read_rom(&mut self, address: u16) -> u8;
    fn write_ram(&mut self, value: u8, address: u16);
    fn write_rom(&mut self, value: u8, address: u16);

    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}
