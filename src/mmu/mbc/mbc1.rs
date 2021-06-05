use crate::mmu::mbc::MBC;
use crate::cartridge::Cartridge;
use std::cmp::{max, min};

pub struct MBC1 {
    mode: u8,

    cartridge: Cartridge,
    rom_bank: u8,
    rom_size: u8,

    ram: Vec<u8>,
    ram_bank: u8,
    ram_enabled: bool,
    ram_mode: bool,
    ram_size: u8,

    bank2: u8,
    bank1: u8
}

impl MBC1 {
    pub fn new(cartridge: Cartridge) -> MBC1 {

        let ram_size = cartridge.rom[0x149];
        let rom_size = cartridge.rom_size();

        let ram = std::iter::repeat(0).take(match ram_size {
            0 => 0,
            2 => 1,
            3 => 4,
            4 => 16,
            5 => 8,
            _ => panic!("impossible")
        } * 0x2000).collect();

        MBC1 {
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


            ram_mode: false,

        }
    }
}

/// Implementation of traits for the basic mbc with no additional banks or ram writes
impl MBC for MBC1 {

    fn read_ram(&mut self, address: u16) -> u8 {
        match self.ram_enabled {
            false => 0,
            true  => 0
        }
    }

    fn read_rom(&mut self, address: u16) -> u8 {
0    }

    fn write_ram(&mut self, value: u8, address: u16) {
    }

    fn write_rom(&mut self, value: u8, address: u16) {
    }

    fn read(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => match self.mode {
                0 => self.cartridge.rom[address as usize],
                1 => {
                    let bank_number = (((self.bank2 as u16) << 5) | (self.bank1 as u16)) as u32;
                    let address = ((bank_number * 0x4000) | (address & 0x3FFF) as u32) as usize % self.cartridge.rom.len();
                    self.cartridge.rom[address]
                },
                _ => panic!("impossible mode: {}", self.mode)
            }
            0x4000..=0x7FFF => {
                self.cartridge.rom[(((self.rom_bank & 0x1F) as u16 * 0x4000) | (address & 0x3FFF)) as usize]
            },
            0xA000..=0xBFFF => match self.mode {
                0 => self.ram[(address & 0x1FFF) as usize],
                1 => self.ram[((self.ram_bank as u16 * 0x2000) | (address & 0x1FFF)) as usize],
                _ => panic!("impossible mode: {}", self.mode)
            },

            _ => panic!("unmapped mbc1 address {:#06X}", address)
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {

            // RAM Enable
            0x0000..=0x1FFF => if self.ram_size != 0 {
                self.ram_enabled = value & 0x0F == 0x0A;
            },

            // ROM Bank Number
            0x2000..=0x3FFF => self.bank1 = {
                let mut masked = value & 0x1F;
                if masked >= (1 << (self.rom_size + 1)) {
                    masked &= (1 << (self.rom_size + 1)) - 1;
                }

                max(1, masked)
            },

            // RAM Bank Number / Upper Bits of ROM Bank Number
            0x4000..=0x5FFF => self.bank2 = value & 0x03,

            // Banking Mode Select
            0x6000..=0x7FFF => self.mode = value & 0x01,

            // RAM Bank 00-03
            0xA000..=0xBFFF => if self.ram_enabled {
                self.ram[((self.ram_bank as u16 * 0x2000) | (address & 0x1FFF)) as usize] = value;
            }

            _ => panic!("unmapped mbc1 address {:#06X}", address)
        }
    }
}
