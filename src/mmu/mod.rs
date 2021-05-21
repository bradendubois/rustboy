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
    w_ram_bank: usize,

    e_ram: Vec<u16>,
    z_ram: Vec<u8>,


    mbc: Box<dyn MBC>
}

#[allow(unreachable_patterns)]
#[allow(dead_code)]
impl MMU {

    pub fn new(rom: Vec<u8>) -> MMU {
        MMU {
            in_bios: false,
            bios: vec![],
            rom: vec![],

            w_ram: [0; W_RAM_SIZE],
            w_ram_bank: 1,

            e_ram: vec![],
            z_ram: vec![],


            mbc: Box::new(match rom[0x147] {
                0x00 ..= 0x00 => MBC0::new(rom),
                0x01 ..= 0x03 => panic!("MBC1 not implemented!"), // MBC1::new(rom),
                0x05 ..= 0x06 => panic!("MBC2 not implemented!"), // MBC2::new(rom),
                0x0F ..= 0x13 => panic!("MBC3 not implemented!"), // MBC3::new(rom),
                0x19 ..= 0x1E => panic!("MBC5 not implemented!"), // MBC5::new(rom)

                _ => panic!("Unsupported cartridge type: {}!", rom[0x147]),
            })
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {

        match address {
            0x0000 ..= 0x3FFF => self.mbc.read_rom(address),    // ROM
            0x4000 ..= 0x7FFF => self.mbc.read_rom(address),    // Switchable ROM Bank
            0x8000 ..= 0x9FFF => 0,                             // Video RAM
            0xA000 ..= 0xBFFF => self.mbc.read_ram(address),    // Switchable RAM Bank
            0xC000 ..= 0xCFFF => self.read_ram(address),        // Internal RAM
            0xD000 ..= 0xDFFF => self.read_rambank(address),    // Internal RAM
            0xE000 ..= 0xEFFF => self.read_ram(address),        // Internal RAM
            0xF000 ..= 0xFDFF => self.read_rambank(address),    // Echo RAM
            0xFE00 ..= 0xFE9F => 0,                             // Sprite Attributes
            0xFEA0 ..= 0xFEFF => 0,                             // Unusable
            0xFF00 ..= 0xFF7F => 0,                             // I/O Registers
            0xFF80 ..= 0xFFFE => 0,                             // High RAM
            0xFFFF ..= 0xFFFF => 0,                             // Interrupt Register

            _ => panic!("Unmapped address {}", address)
        }
    }

    pub fn write(&mut self, value: u8, address: u16) {

        match address {
            0x0000 ..= 0x3FFF => self.mbc.write_rom(value, address),    // ROM
            0x4000 ..= 0x7FFF => self.mbc.write_rom(value, address),    // Switchable ROM Bank
            0x8000 ..= 0x9FFF => (),                                    // Video RAM
            0xA000 ..= 0xBFFF => self.mbc.write_ram(value, address),    // Switchable RAM Bank
            0xC000 ..= 0xCFFF => self.write_ram(value, address),        // Internal RAM
            0xD000 ..= 0xDFFF => self.write_rambank(value, address),    // Internal RAM
            0xE000 ..= 0xEFFF => self.write_ram(value, address),        // Internal RAM
            0xF000 ..= 0xFDFF => self.write_rambank(value, address),    // Echo RAM
            0xFE00 ..= 0xFE9F => (),                                    // Sprite Attributes
            0xFEA0 ..= 0xFEFF => (),                                    // Unusable
            0xFF00 ..= 0xFF7F => (),                                    // I/O Registers
            0xFF80 ..= 0xFFFE => (),                                    // High RAM
            0xFFFF ..= 0xFFFF => (),                                    // Interrupt Register

            _ => panic!("Unmapped address {}", address)
        };
    }

    fn w_ram_conv(address: u16) -> usize {
        address as usize & 0x0FFF
    }

    fn w_rambank_conv(&self, address: u16) -> usize {
        MMU::w_ram_conv(address) | (self.w_ram_bank * 0x1000)
    }

    fn read_ram(&mut self, address: u16) -> u8 {
        self.w_ram[MMU::w_ram_conv(address)]
    }

    fn read_rambank(&mut self, address: u16) -> u8 {
        self.w_ram[self.w_rambank_conv(address)]
    }

    fn write_ram(&mut self, value: u8, address: u16) {
        self.w_ram[MMU::w_ram_conv(address)] = value;
    }

    fn write_rambank(&mut self, value: u8, address: u16) {
        self.w_ram[self.w_rambank_conv(address)] = value;
    }
}


impl fmt::Debug for MMU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MMU Debug")
    }
}
