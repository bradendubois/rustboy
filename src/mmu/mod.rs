#[derive(Debug)]
pub struct MMU {

    in_bios: bool,
    bios: Vec<u8>,
    rom: Vec<u16>,
    w_ram: Vec<u16>,
    e_ram: Vec<u16>,
    z_ram: Vec<u8>,

    memory: Vec<u8>

}

#[allow(dead_code)]
impl MMU {

    pub fn init() -> MMU {
        MMU {

            in_bios: false,
            bios: vec![],
            rom: vec![],
            w_ram: vec![],
            e_ram: vec![],
            z_ram: vec![],

            memory: vec![0; 1 << 16]
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write(&mut self, value: u8, address: u16) {
        self.memory[address as usize] = value;
    }
}
