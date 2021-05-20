use crate::mmu::mbc::MBC;

pub struct MBC0 {
    rom: Vec<u8>
}

impl MBC0 {
    pub fn new(data: Vec<u8>) -> MBC0 {
        MBC0 {
            rom: data
        }
    }
}

/// Implmentation of traits for the basic mbc with no additional banks or ram writes
impl MBC for MBC0 {
    fn read_ram(&self, _address: u16) -> u8 {
        0
    }

    fn read_rom(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn write_ram(&mut self, _value: u8, _address: u16) { }

    fn write_rom(&mut self, _value: u8, _address: u16) { }
}
