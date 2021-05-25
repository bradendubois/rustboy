const V_RAM_SIZE: usize = 0x2000;
const   OAM_SIZE: usize = 0x0100;

#[allow(dead_code)]
const  TILE_SIZE: usize = 128;

/// GameBoy Screen Height
#[allow(dead_code)]
pub const HEIGHT: usize = 144;

/// GameBoy Screen Width
#[allow(dead_code)]
pub const WIDTH: usize = 160;

#[allow(dead_code)]
enum Mode {
    Mode0,  // HBlank Period
    Mode1,  // VBlank Period
    Mode2   // Searching OAM Period
}


pub struct PPU {
    #[allow(dead_code)]
    mode: Mode,                 // PPU Mode
    vram: [u8; V_RAM_SIZE],     // VRAM
     oam: [u8; OAM_SIZE],       // OAM / Sprite Attribute Table
    lcd_enabled: bool,          // Status of the PPU / LCD ; true = on, false = off

    lcdc: u8,       // 0xFF40 : LCDC Register : LCD C(ontrol) Register
    lcds: u8,       // 0xFF41 : LCDS Register : LCD S(tatus) Register
     scy: u8,       // 0xFF42 : Scroll Y
     scx: u8,       // 0xFF43 : Scroll X
      ly: u8,       // 0xFF44 : LY  (LCD Y)
     lyc: u8,       // 0xFF45 : LYC (LY Compare)
     bgp: u8,       // 0xFF47 : BGP Palette Data (Non-CGB)
    obp0: u8,       // 0xFF48 : Object Palette 0 (Non-CGB)
    obp1: u8,       // 0xFF49 : Object Palette 1 (Non-CGB)
      wy: u8,       // 0xFF4A : Window Y Position
      wx: u8        // 0xFF4B : Window X Position
}

impl PPU {

    pub fn new() -> PPU {
        PPU {
            mode: Mode::Mode0,
            vram: [0; V_RAM_SIZE],
             oam: [0; OAM_SIZE],
            lcd_enabled: true,

            lcdc: 0,
            lcds: 0,
             scy: 0,
             scx: 0,
              ly: 0,
             lyc: 0,
             bgp: 0,
            obp0: 0,
            obp1: 0,
              wy: 0,
              wx: 0
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {

        match address {
            0x8000 ..= 0x9FFF => self.vram[PPU::addr_into_vram_space(address)],
            0xFE00 ..= 0xFE9F => self.oam[PPU::addr_into_oam_space(address)],
            0xFF40 => self.lcdc,
            0xFF41 => self.lcds,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,

            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,

            0xFF4A => self.wy,
            0xFF4B => self.wx,

            _ => panic!("unmapped address: {}", address)
        }
    }

    pub fn write(&mut self, value: u8, address: u16) {

        match address {
            0x8000 ..= 0x9FFF => self.vram[PPU::addr_into_vram_space(address)] = value,
            0xFE00 ..= 0xFE9F => self.oam[PPU::addr_into_oam_space(address)] = value,
            0xFF40 => self.lcdc = value,
            0xFF41 => self.lcds = value,
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF44 => (),   // read-only
            0xFF45 => self.lyc = value,
            0xFF47 => self.bgp = value,
            0xFF48 => self.obp0 = value,
            0xFF49 => self.obp1 = value,
            0xFF4A => self.wy = value,
            0xFF4B => self.wx = value,

            _ => panic!("unmapped address: {}", address)
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

    /// LYC = LY STATE Interrupt Source
    pub fn lcds_lyc_int(&self) -> bool {self.lcds & 0x40 == 0x40}

    /// Mode 2 OAM STAT Interrupt Source
    pub fn lcds_oam_int(&self) -> bool {self.lcds & 0x20 == 0x20}

    /// Mode 1 VBlank STAT Interrupt source
    pub fn lcds_vblank_int(&self) -> bool {self.lcds & 0x10 == 0x10}

    /// Mode 0 HBlank STAT Interrupt source
    pub fn lcds_hblank_int(&self) -> bool {self.lcds & 0x08 == 0x08}

    /// LYC = LY Flag
    pub fn lcds_lyc_ly(&self) -> bool {self.lcds & 0x04 == 0x04}

    /// Mode Flag
    pub fn lcds_mode_flag(&self) -> u8 {
        (self.lcds << 6 ) >> 6
    }
}