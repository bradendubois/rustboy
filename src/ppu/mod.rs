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

    pub fn read(&mut self, address: u16) -> u8 {

        match address {
            0x8000 ..= 0x9FFF => self.vram[PPU::addr_into_vram_space(address)],
            0xFE00 ..= 0xFE9F => self.oam[PPU::addr_into_oam_space(address)],
            0xFF40 => self.lcdc,
            0xFF41 => self.lcds
        }
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

    /// Check if the LCD is ON and PPU is active
    /// Returns true if the register bit is set and false otherwise
    pub fn lcdc_lcd_enable(&self) -> bool{self.lcdc & 0x80 == 0x80}

    /// Window tile map area
    /// if the bit is set the tile map area is 9C00-9FFF otherwise it's 9800-9BFF
    /// Controls the background map the window uses for rendering
    pub fn lcdc_window_tile_map_area(&self) -> bool{self.lcdc & 0x40 == 0x40}

    /// Window enable
    /// Controls whether the window shall be displayed or not.
    pub fn lcdc_window_enabled(&self) -> bool{self.lcdc & 0x20 == 0x20}

    /// BG and Window Tile Data Area
    /// controls which addressing mode the BG and Window use to pick tiles
    /// 0 = 9800 - 9BFF, 1 = 9C00 - 9FFF
    pub fn lcdc_bg_window_tile_area(&self) -> bool{self.lcdc & 0x10 == 0x10}

    /// BG Tile Map Area
    /// Similar to the window tile map area
    /// if the bit is set the tile map area is 9C00-9FFF otherwise it's 9800-9BFF
    pub fn lcdc_bg_tile_map_area(&self) -> bool{self.lcdc & 0x08 == 0x08}

    /// OBJ Size
    /// Controls sprite size
    /// 0 = 1 tile, 1 = 2 tiles (stacked vertically)
    pub fn lcdc_obj_size(&self) -> bool{self.lcdc & 0x04 == 0x04}

    /// OBJ Enable
    /// Toggles whether sprites are displayed or not
    pub fn lcdc_obj_enable(&self) -> bool{self.lcdc & 0x02 == 0x02}

    /// BG and Window enable/priority
    /// This has different meanings depending on the gameboy type and mode
    /// [For more Info](https://gbdev.io/pandocs/LCDC.html#lcdc0---bg-and-window-enablepriority)
    pub fn lcdc_zero_bit(&self) -> bool{self.lcdc & 0x01 == 0x01}

}