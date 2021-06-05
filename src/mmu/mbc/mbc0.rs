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

    fn read(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.cartridge.rom[address as usize],
            0xA000..=0xBFFF => 0,
            _ => panic!("address not implemented for mbc0: {:#06X}", address)
        }
    }

    fn write(&mut self, address: u16, value: u8) { }
}
