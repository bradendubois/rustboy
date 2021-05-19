#[derive(Debug)]
pub struct MMU {
    in_bios: bool,
    bios: Vec<u8>,
    rom: Vec<u16>,
    w_ram: Vec<u16>,
    e_ram: Vec<u16>,
    z_ram: Vec<u8>,

    memory: Vec<u8>,
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

            memory: vec![0; 1 << 16],
        }
    }

    pub fn read(&self, address: u16) -> u8 {

        match address {
            0x0000 ..= 0x3FFF => 0,     // ROM
            0x4000 ..= 0x7FFF => 0,     // Switchable ROM Bank
            0x8000 ..= 0x9FFF => 0,     // Video RAM
            0xA000 ..= 0xBFFF => 0,     // Switchable RAM Bank
            0xC000 ..= 0xDFFF => 0,     // Internal RAM
            0xE000 ..= 0xFDFF => 0,     // Unusable
            0xFE00 ..= 0xFE9F => 0,     // Sprite Attributes
            0xFEA0 ..= 0xFEFF => 0,     // Unusable
            0xFF00 ..= 0xFF4B => 0,     // I/O
            0xFF4C ..= 0xFF7F => 0,     // Unusable
            0xFF80 ..= 0xFFFE => 0,     // High RAM
            0xFFFF => 0,                // Interrupt Register

            _ => panic!("Unmapped address {}", address)
        }
    }

    pub fn write(&mut self, value: u8, address: u16) {

        match address {
            0x0000 ..= 0x3FFF => 0,     // ROM
            0x4000 ..= 0x7FFF => 0,     // Switchable ROM Bank
            0x8000 ..= 0x9FFF => 0,     // Video RAM
            0xA000 ..= 0xBFFF => 0,     // Switchable RAM Bank
            0xC000 ..= 0xDFFF => 0,     // Internal RAM
            0xE000 ..= 0xFDFF => 0,     // Unusable
            0xFE00 ..= 0xFE9F => 0,     // Sprite Attributes
            0xFEA0 ..= 0xFEFF => 0,     // Unusable
            0xFF00 ..= 0xFF4B => 0,     // I/O
            0xFF4C ..= 0xFF7F => 0,     // Unusable
            0xFF80 ..= 0xFFFE => 0,     // High RAM
            0xFFFF => 0,                // Interrupt Register

            _ => panic!("Unmapped address {}", address)
        };
    }
}
