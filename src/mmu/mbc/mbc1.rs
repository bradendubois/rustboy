use std::cmp::max;
use crate::cartridge::Cartridge;
use super::{MBC, create_ram, ROM_BANK_SIZE, RAM_BANK_SIZE};


pub struct MBC1 {
    mode: u8,

    cartridge: Cartridge,
    rom_size: u8,
    bank1: u8,
    bank2: u8,

    ram: Vec<u8>,
    ram_enabled: bool,
    ram_size: u8,
}


impl MBC1 {
    pub fn new(cartridge: Cartridge) -> MBC1 {

        let ram_size = cartridge.rom[0x149];
        let rom_size = cartridge.rom_size();

        let ram = create_ram(ram_size);

        MBC1 {
            mode: 0,
            cartridge,
            rom_size,
            bank1: 1,
            bank2: 0,

            ram,
            ram_enabled: false,
            ram_size
        }
    }
}


impl MBC for MBC1 {

    fn read(&mut self, address: u16) -> u8 {
        match address {
            // ROM Read - Mode 0 -> Bank 0, Mode 1 -> Bank2 << 5
            0x0000..=0x3FFF => match self.mode {
                0 => self.cartridge.rom[address as usize],
                1 => {
                    let address = (ROM_BANK_SIZE * ((self.bank2 << 5) as usize)) | ((address as usize) & ROM_BANK_SIZE - 1);
                    self.cartridge.rom[address % self.cartridge.rom.len()]
                },
                _ => panic!("impossible mode: {}", self.mode)
            }

            // ROM Read - Banked
            0x4000..=0x7FFF => {
                let address = (ROM_BANK_SIZE * (((self.bank2 << 5) | self.bank1) as usize)) | ((address as usize) & ROM_BANK_SIZE - 1);
                self.cartridge.rom[address % self.cartridge.rom.len()]
            },

            // RAM Read
            0xA000..=0xBFFF => match self.ram_enabled {

                // RAM Disabled
                false => 0x00,

                // RAM Enabled
                true  => match self.mode {
                    0 => self.ram[(address as usize) & (RAM_BANK_SIZE - 1)],
                    1 => self.ram[(self.bank2 as usize * RAM_BANK_SIZE) | ((address as usize) & (RAM_BANK_SIZE - 1))],
                    _ => panic!("impossible mode: {}", self.mode)
                }
            }
            _ => panic!("unmapped mbc1 address {:#06X}", address)
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {

            // RAM Enable
            0x0000..=0x1FFF => if self.ram_size != 0 { self.ram_enabled = value & 0x0F == 0x0A; },

            // ROM Bank Number / Lower bits
            0x2000..=0x3FFF => self.bank1 = {
                let mut masked = (value & 0x1F) as u16;
                if masked >= (1 << (self.rom_size + 1)) {
                    masked &= (1 << (self.rom_size + 1)) - 1;
                }

                max(1, masked as u8)
            },

            // RAM Bank Number / Upper Bits of ROM Bank Number
            0x4000..=0x5FFF => self.bank2 = value & 0x03,

            // Banking Mode Select
            0x6000..=0x7FFF => self.mode = value & 0x01,

            // RAM Bank 00-03
            0xA000..=0xBFFF => if self.ram_enabled {
                match self.mode {
                    0 => self.ram[(address as usize) & (RAM_BANK_SIZE - 1)] = value,
                    1 => self.ram[((self.bank2 as usize) * RAM_BANK_SIZE) | (address as usize & (RAM_BANK_SIZE - 1))] = value,
                    _ => panic!("impossible mode value: {}", self.mode)
                }
            }

            _ => panic!("unmapped mbc1 address {:#06X}", address)
        }
    }
}


#[cfg(test)]
mod test {

    use crate::testing::mooneye_all;

    #[test]
    fn acceptance_mbc1() {
        mooneye_all("emulator-only/mbc1");
    }
}
