use crate::cartridge::Cartridge;
use super::{MBC, ROM_BANK_SIZE};

const RAM_SIZE: usize = 0x200;

#[allow(dead_code)]
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

#[allow(dead_code)]
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
                self.cartridge.rom[(address & (ROM_BANK_SIZE as u16 - 1)) as usize]
            },
            0xA000..=0xBFFF => self.ram[(address & ((1<<9) - 1) & (RAM_SIZE as u16 -1))as usize],

            _ => panic!("unmapped mbc2 address {:#06X}", address)
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        let upper_byte: u8 = (address >> 8) as u8;
        let lower_byte: u8 = address as u8;
        // If the address's third least significant bit is even it controls the ram
        // otherwise it controls the rom
        if address & 0x100 % 0x200 == 0{

        }

        match address {
            //Addresses that can write to rom are
            // address & 0x100 mod 200 != 0
            // Addresses that can write to ram are mod 200 == 0
            // RAM Enable
            0x0000..=0x3FFF => {
                if upper_byte & 0x01 == 1 {
                    self.rom_bank = value &0x0f;
                    self.ram[(lower_byte & 0x0f) as usize] = value
                }
            else {
                if value == 0x0A{self.ram_enabled = true;}else{self.ram_enabled=false;}
            }
            },
            0xA000 ..=0xBFFF => {
                self.ram[(address & ((1<<9)-1)& (RAM_SIZE as u16 -1)) as usize] = value;
            },

            _ => panic!("unmapped mbc2 address {:#06X}", address)
        }
    }
}

#[cfg(test)]
mod test{
    use crate::testing::mooneye_all;

    #[test]
    fn acceptance_mbc2() {mooneye_all("emulator-only/mbc2");}
}