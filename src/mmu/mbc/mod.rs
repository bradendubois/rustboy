
pub trait MBC {
    fn read_ram(&mut self, address: u16) -> u8;
}


pub struct MBC0 {
    rom: Vec<u8>,
}

impl MBC0 {
    pub fn new(data: Vec<u8>) -> MBC0 {
        MBC0 {
            rom: data
        }
    }
}

impl MBC for MBC0 {
    fn read_ram(&mut self, address: u16) -> u8 {
        self.rom[address as usize]
    }
}