
pub trait MBC {
    fn read_ram(&self, address: u16) -> u8;
    fn read_rom(&self, address: u16) -> u8;
    fn write_ram(&mut self, value: u8, address: u16);
    fn write_rom(&mut self, value: u8, address: u16);
}


pub struct MBC0 {
    rom: Vec<u8>
}


impl MBC0 {
    pub fn new(data: Vec<u8>) -> MBC0 {
        MBC0 {
            rom: data
        }
    }
}

impl MBC for MBC0 {
    fn read_ram(&self, _address: u16) -> u8 {
        0
    }

    fn read_rom(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn write_ram(&mut self, _value: u8, _address: u16) {

    }

    fn write_rom(&mut self, _value: u8, _address: u16) {

    }
}