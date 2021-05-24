const V_RAM_SIZE: usize = 0x2000;
const   OAM_SIZE: usize = 0x0100;
const  TILE_SIZE: usize = 128;

/// GameBoy Screen Height
pub const HEIGHT: usize = 144;

/// GameBoy Screen Width
pub const WIDTH: usize = 160;

enum Mode {
    Mode0,  // HBlank Period
    Mode1,  // VBlank Period
    Mode2   // Searching OAM Period
}

pub struct PPU {
    mode: Mode,                 // PPU Mode
    vram: [u8; V_RAM_SIZE],     // VRAM
     oam: [u8; OAM_SIZE],       // OAM / Sprite Attribute Table
    lcdc:  u8,                  // LCDC Register : LCD C(ontrol) Register   ; 0xFF40
    lcds:  u8,                  // LCDS Register : LCD S(tatus) Register    ; 0xFF41
    lcd_enabled: bool           // Status of the PPU / LCD ; true = on, false = off
}

impl PPU {

    pub fn new() -> PPU {
        PPU {
            mode: Mode::Mode0,
            vram: [0; V_RAM_SIZE],
             oam: [0; OAM_SIZE],
            lcdc:  0,
            lcds:  0,
            lcd_enabled: true
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

    fn addr_into_vram_space(address: u16) -> usize {
        address as usize - 0x8000
    }

    fn addr_into_oam_space(address: u16) -> usize {
        address as usize - 0xFE00
    }

    pub fn read_lcdc(&self) -> u8 {
        self.lcdc
    }

    pub fn write_lcdc(&mut self, value: u8) {

        match (value & 0x80) >> 7 {
            0 => self.lcd_enabled = false,
            1 => self.lcd_enabled = true,
            _ => panic!("arithmetic failure")
        }

        self.lcdc = value;
    }


    /*
       Potentially unused code. May be useful for the
       graphical implementations we create later.
     */
    #[allow(dead_code)]
    pub fn get_tile_set_1(&self) -> Vec<u8> {
        self.vram[0..0x07ff].to_vec()
    }
    #[allow(dead_code)]
    pub fn get_tile_set_1_and_0(&self) -> Vec<u8> {
        self.vram[0x0800..0x0FFF].to_vec()
    }
    #[allow(dead_code)]
    pub fn get_tile_set_0(&self) -> Vec<u8> {
        self.vram[0x1000..0x17ff].to_vec()
    }
    #[allow(dead_code)]
    pub fn get_tile_map_0(&self) -> Vec<u8> {
        self.vram[0x1800..0x1bff].to_vec()
    }
    #[allow(dead_code)]
    pub fn get_tile_map_1(&self) -> Vec<u8> {
        self.vram[0x1c00..0x1FFF].to_vec()
    }
}