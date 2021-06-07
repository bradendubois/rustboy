use crate::cartridge::Cartridge;
use super::{MBC, create_ram, ROM_BANK_SIZE, RAM_BANK_SIZE};


pub struct MBC5 {
    cartridge: Cartridge,
    bank1: u8,
    bank2: u8,

    ram: Vec<u8>,
    ram_enabled: bool,
    ram_bank: usize,

    rumble: bool
}


impl MBC5 {
    pub fn new(cartridge: Cartridge) -> MBC5 {

        let ram = create_ram(cartridge.rom[0x149]);

        MBC5 {
            cartridge,
            bank1: 1,
            bank2: 0,

            ram,
            ram_enabled: false,
            ram_bank: 0,

            rumble: false
        }
    }
}


impl MBC for MBC5 {

    fn read(&mut self, address: u16) -> u8 {
        match address {

            // ROM Read - Bank 0
            0x0000..=0x3FFF => self.cartridge.rom[address as usize],

            // ROM Read - Banked
            0x4000..=0x7FFF => {
                let bank_number = (((self.bank2 as u16) << 8) | self.bank1 as u16) as usize;
                self.cartridge.rom[ROM_BANK_SIZE * bank_number | (address as usize & (ROM_BANK_SIZE - 1))]
            },

            // RAM Read
            0xA000..=0xBFFF => match self.ram_enabled {

                // RAM Disabled
                false => 0x00,

                // RAM Enabled
                true  => self.ram[RAM_BANK_SIZE * self.ram_bank | ((address as usize) & (RAM_BANK_SIZE - 1))]
            }
            _ => panic!("unmapped MBC5 address {:#06X}", address)
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {

            // RAM Enable
            0x0000..=0x1FFF => self.ram_enabled = value == 0x0A,

            // ROM Bank Number / Lower 8 bits
            0x2000..=0x2FFF => self.bank1 = value,

            // ROM Bank Number / 9th bit
            0x3000..=0x3FFF => self.bank2 = value & 0x01,

            // RAM Bank Number
            0x4000..=0x5FFF => {
                self.rumble = (value & 0x04) != 0;
                self.ram_bank = (value & 0x03) as usize
            },

            // RAM Bank 00-03
            0xA000..=0xBFFF => if self.ram_enabled {
                self.ram[RAM_BANK_SIZE * self.ram_bank | ((address as usize) & (RAM_BANK_SIZE - 1))] = value
            }

            _ => panic!("unmapped MBC5 address {:#06X}", address)
        }
    }
}


#[cfg(test)]
mod test {

    use crate::testing::mooneye_all;

    #[test]
    fn acceptance_mbc5() {
        mooneye_all("emulator-only/mbc5");
    }
}