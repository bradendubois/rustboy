use crate::cartridge::Cartridge;
use crate::traits::MBC;

pub mod mbc0;
pub mod mbc1;

const ROM_BANK_SIZE: usize = 0x4000;
const RAM_BANK_SIZE: usize = 0x2000;


// Create RAM for an MBC given the RAM size code in the cartridge
pub fn create_ram(ram_size: u8) -> Vec<u8> {
    std::iter::repeat(0).take(match ram_size {
        0 => 0,
        2 => 1,
        3 => 4,
        4 => 16,
        5 => 8,

        _ => panic!("impossible ram size: {}", ram_size)
    } * (RAM_BANK_SIZE as usize)).collect()
}

pub fn from(cartridge: Cartridge) -> Box<dyn MBC> {
    match cartridge.cartridge_type() {
        0x00 ..= 0x00 => Box::new(mbc0::MBC0::new(cartridge)),
        0x01 ..= 0x03 => Box::new(mbc1::MBC1::new(cartridge)),
        0x05 ..= 0x06 => panic!("MBC2 not implemented!"), // MBC2::new(cartridge),
        0x0F ..= 0x13 => panic!("MBC3 not implemented!"), // MBC3::new(cartridge),
        0x19 ..= 0x1E => panic!("MBC5 not implemented!"), // MBC5::new(cartridge)

        _ => panic!("Unsupported cartridge type: {:#4X}!", cartridge.cartridge_type()),
    }
}