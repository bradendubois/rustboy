use crate::mmu::mbc::MBC;
use crate::cartridge::Cartridge;
use std::cmp::max;

pub struct MBC1 {
    cartridge: Cartridge,
    rom_bank: u16,

    ram: Vec<u8>,
    ram_bank: u16,
    ram_enabled: bool,
    ram_mode: bool
}

impl MBC1 {
    pub fn new(cartridge: Cartridge) -> MBC1 {

        let ram_size = cartridge.rom[0x149] as usize;

        MBC1 {
            cartridge,
            rom_bank: 1,

            ram: std::iter::repeat(0).take(ram_size).collect(),
            ram_bank: 0,
            ram_enabled: false,
            ram_mode: false
        }
    }
}

/// Implementation of traits for the basic mbc with no additional banks or ram writes
impl MBC for MBC1 {

    fn read_ram(&mut self, address: u16) -> u8 {
        match self.ram_enabled {
            false => 0,
            true  => {
                match self.ram_mode {
                    true  => self.ram[((self.ram_bank * 0x2000) as u16 | (address & 0x1FFF)) as usize],
                    false => self.ram[(address & 0x1FFF) as usize]
                }
            }
        }
    }

    fn read_rom(&mut self, address: u16) -> u8 {
        if address < 0x4000 {
            return self.cartridge.rom[address as usize];
        }

        let rombank_address = self.rom_bank * 0x4000 | (address & 0x3FFF);
        self.cartridge.rom[rombank_address as usize]
    }

    fn write_ram(&mut self, value: u8, address: u16) {
        if self.ram_enabled {
            let ram_bank = if self.ram_mode { self.ram_bank } else { 0 };
            self.ram[((ram_bank * 0x2000) as usize | (address & 0x1FFF) as usize)] = value
        }
    }

    fn write_rom(&mut self, value: u8, address: u16) {
        match address {
            0x0000..=0x1FFF => self.ram_enabled = value == 0x0A,
            0x2000..=0x3FFF => self.rom_bank = max(1, (value & 0x1F) as u16),
            0x4000..=0x5FFF => {
                match self.ram_mode {
                    true  => self.ram_bank = value as u16 & 0x03,
                    false => self.rom_bank = (self.rom_bank & 0x1F) | (((value as usize) & 0x03) << 5) as u16
                }
            },
            0x6000..=0x7FFF => self.ram_mode = value & 0x01 == 0x01,

            _ => panic!("invalid address: {}", address)
        }
    }
}
