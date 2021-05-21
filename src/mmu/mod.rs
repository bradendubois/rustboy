mod mbc;

use mbc::{MBC, mbc0::{MBC0}};
use std::fmt;

const W_RAM_SIZE: usize = 0x8000;

#[allow(unreachable_patterns)]
pub struct MMU {
    in_bios: bool,
    bios: Vec<u8>,
    rom: Vec<u16>,

    w_ram: [u8; W_RAM_SIZE],
    e_ram: Vec<u16>,
    z_ram: Vec<u8>,


    mbc: Box<dyn MBC>
}

#[allow(unreachable_patterns)]
#[allow(dead_code)]
impl MMU {
    pub fn init() -> MMU {
        MMU {
            in_bios: false,
            bios: vec![],
            rom: vec![],

            w_ram: [0; W_RAM_SIZE],
            e_ram: vec![],
            z_ram: vec![],


            mbc: Box::new(MBC0::new(vec![]))
        }
    }

    pub fn read(&self, address: u16) -> u8 {

        match address {
            0x0000 ..= 0x3FFF => self.mbc.read_rom(address),    // ROM
            0x4000 ..= 0x7FFF => self.mbc.read_rom(address),    // Switchable ROM Bank
            0x8000 ..= 0x9FFF => 0,                             // Video RAM
            0xA000 ..= 0xBFFF => self.mbc.read_ram(address),    // Switchable RAM Bank
            0xC000 ..= 0xDFFF => 0,                             // Internal RAM
            0xE000 ..= 0xFDFF => 0,                             // Unusable
            0xFE00 ..= 0xFE9F => 0,                             // Sprite Attributes
            0xFEA0 ..= 0xFEFF => 0,                             // Unusable
            0xFF00 ..= 0xFF4B => 0,                             // I/O
            0xFF4C ..= 0xFF7F => 0,                             // Unusable
            0xFF80 ..= 0xFFFE => 0,                             // High RAM
            0xFFFF => 0,                                        // Interrupt Register

            _ => panic!("Unmapped address {}", address)
        }
    }

    pub fn write(&mut self, value: u8, address: u16) {

        match address {
            0x0000 ..= 0x3FFF => self.mbc.write_rom(value, address),    // ROM
            0x4000 ..= 0x7FFF => self.mbc.write_rom(value, address),    // Switchable ROM Bank
            0x8000 ..= 0x9FFF => (),                                    // Video RAM
            0xA000 ..= 0xBFFF => (),                                    // Switchable RAM Bank
            0xC000 ..= 0xDFFF => self.mbc.write_ram(value, address),    // Internal RAM
            0xE000 ..= 0xFDFF => (),                                    // Unusable
            0xFE00 ..= 0xFE9F => (),                                    // Sprite Attributes
            0xFEA0 ..= 0xFEFF => (),                                    // Unusable
            0xFF00 ..= 0xFF4B => (),                                    // I/O
            0xFF4C ..= 0xFF7F => (),                                    // Unusable
            0xFF80 ..= 0xFFFE => (),                                    // High RAM
            0xFFFF => (),                // Interrupt Register

            _ => panic!("Unmapped address {}", address)
        };
    }
}


impl fmt::Debug for MMU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MMU Debug")
    }
}
