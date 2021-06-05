pub mod mbc0;
pub mod mbc1;

const ROM_BANK_SIZE: u16 = 0x4000;
const RAM_BANK_SIZE: u16 = 0x2000;


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

/// The traits that any memory bank controller (MBC) must implement
pub trait MBC {
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}
