use crate::mmu::mbc::MBC;
use crate::cartridge::Cartridge;

pub struct MBC0 {
    cartridge: Cartridge
}

impl MBC0 {
    pub fn new(cartridge: Cartridge) -> MBC0 {
        MBC0 {
            cartridge
        }
    }
}

/// Implementation of traits for the basic mbc with no additional banks or ram writes
impl MBC for MBC0 {

    fn read_ram(&mut self, _address: u16) -> u8 {
        0
    }

    fn read_rom(&mut self, address: u16) -> u8 {
        self.cartridge.rom[address as usize]
    }

    fn write_ram(&mut self, _value: u8, _address: u16) { }

    fn write_rom(&mut self, _value: u8, _address: u16) { }
}
