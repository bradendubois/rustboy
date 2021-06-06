use std::cmp::max;
use crate::cartridge::Cartridge;
use super::{MBC, create_ram, ROM_BANK_SIZE, RAM_BANK_SIZE};
use sdl2::gfx::imagefilter::add;

const RAM_SIZE: usize = 0x200;
pub struct MBC2 {
    mode: u8,

    cartridge: Cartridge,
    rom_bank: u8,
    rom_size: u8,

    ram: [u8; RAM_SIZE],
    ram_enabled: bool,
    ram_bank: u8,
    ram_size: u8,

    bank2: u8,
    bank1: u8

}

impl MBC2 {
    pub fn new(cartridge: Cartridge) -> MBC2 {

        let ram_size = cartridge.rom[0x149];
        let rom_size = cartridge.rom_size();

        let ram = [0;RAM_SIZE];


        MBC2 {
            mode: 0,

            cartridge,
            rom_bank: 1,
            rom_size,

            ram,
            ram_bank: 0,
            ram_enabled: false,
            ram_size,

            bank2: 0,
            bank1: 0,
        }
    }
}

impl MBC for MBC2 {

    fn read(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.cartridge.rom[address as usize],
            0x4000..=0x7FFF => {
                self.cartridge.rom[(address & (ROM_BANK_SIZE - 1)) as usize]
            },
            0xA000..=0xBFFF => self.ram[(address & ((1<<9) - 1) & (RAM_SIZE-1))],

            _ => panic!("unmapped mbc2 address {:#06X}", address)
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {

            // RAM Enable
            0x0000..=0x3FFF => if self.ram_size != 0 { self.ram_enabled = value & 0x0F == 0x0A; },

            // ROM Bank Number
            0xA000..=0xA1FF => if self.ram_enabled{
                self.ram[(address & (RAM_BANK_SIZE -1)) as usize]
            },

            // RAM Bank Number / Upper Bits of ROM Bank Number
            0x4000..=0x5FFF => self.bank2 = value & 0x03,

            // Banking Mode Select
            0x6000..=0x7FFF => self.mode = value & 0x01,

            // RAM Bank 00-03
            0xA000..=0xBFFF => if self.ram_enabled {
                self.ram[(address & ((1<<9) - 1) & (RAM_SIZE-1))] = value;
            }

            _ => panic!("unmapped mbc2 address {:#06X}", address)
        }
    }
}