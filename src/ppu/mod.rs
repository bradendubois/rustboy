const V_RAM_SIZE: usize = 0x2000;
const   OAM_SIZE: usize = 0x0100;

enum Mode {
    Mode0,  // HBlank Period
    Mode1,  // VBlank Period
    Mode2   // Searching OAM Period
}


pub struct PPU {
    mode: Mode,                 // PPU Mode
    vram: [u8; V_RAM_SIZE],     // VRAM
     oam: [u8; OAM_SIZE],       // OAM / Sprite Attribute Table
    lcdc: u8                    // LCDC Register : LCD C(ontrol) Register
}

impl PPU {

    pub fn new() -> PPU {
        PPU {
            mode: Mode::Mode0,
            vram: [0; V_RAM_SIZE],
             oam: [0; OAM_SIZE],
            lcdc: 0,
        }
    }

    pub fn read_vram(&mut self, address: u16) -> u8 {
        let _true_address = PPU::addr_into_vram_space(address);
        0
    }

    pub fn write_vram(&mut self, value: u8, address: u16) {
        let _true_address = PPU::addr_into_vram_space(address);
    }

    pub fn read_oam(&mut self, address: u16) -> u8 {
        0
    }

    pub fn write_oam(&mut self, value: u8, address: u16) {
        let _true_address = PPU::addr_into_vram_space(address);
    }

    fn addr_into_vram_space(address: u16) -> u16 {
        address % 0x8000
    }
}