mod byte;
mod mode;

use crate::ppu::byte::Byte;

use byte::{
    lcdc::LCDC,
    lcds::LCDS
};

use mode::Mode;


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
pub struct PPU {
    mode: Mode,                 // PPU Mode
    vram: [u8; V_RAM_SIZE],     // VRAM
     oam: [u8; OAM_SIZE],       // OAM / Sprite Attribute Table

    lcdc: LCDC,     // 0xFF40 : lcdc Register : LCD C(ontrol) Register
    lcds: LCDS,     // 0xFF41 : LCDS Register : LCD S(tatus) Register
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


#[allow(dead_code)]
struct OAMFlags {
    priority: u8,   // OBJ-to-BG Priority   (0 = above BG,      1 = behind BG colors 1 - 3)
      flip_y: u8,   // Y Flip               (0 = normal,        1 = vertical mirror)
      flip_x: u8,   // X Flip               (0 = normal,        1 = horizontal mirror)
     palette: u8    // Object Palette       (0 = obp0 @ 0xFF48, 1 = obp1 @ 0xFF49)
}

/// An entry in the OAM table for a sprite
#[allow(dead_code)]
pub struct OAMEntry {
    position_y: u8,     // Position X
    position_x: u8,     // Position Y
    tile_number: u8,    // Tile Number
    flags: OAMFlags     // Flags
}


#[allow(dead_code)]
impl PPU {

    pub fn new() -> PPU {
        PPU {
            mode: Mode::Mode0,
            vram: [0; V_RAM_SIZE],
             oam: [0; OAM_SIZE],

            lcdc: LCDC::new(),
            lcds: LCDS::new(),
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
            0xFF40 => self.lcdc.read(),
            0xFF41 => self.lcds.read(),
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

        // TODO - Add checks on VRAM / OAM against MODE to ensure access is possible
        match address {
            0x8000 ..= 0x9FFF => self.vram[PPU::addr_into_vram_space(address)] = value,
            0xFE00 ..= 0xFE9F => self.oam[PPU::addr_into_oam_space(address)] = value,
            0xFF40 => self.lcdc.write(value),
            0xFF41 => self.lcds.write(value),
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

    fn draw_screen(&mut self) {

        // TODO - Flesh this out as we read

        // clear_screen ?

        // draw background ?

        // draw sprites ?

    }

    fn oam_entry(&mut self, entry_number: u8) -> OAMEntry {
        assert!(entry_number < 40, "asking for entry number beyond 40");

        // OAM entries are aligned on 4-byte boundaries beginning at 0xFE00
        let oam_address: u16 = 0xFE00 + (entry_number * 4) as u16;
        let flags = self.read(oam_address + 3);

        OAMEntry {
             position_y: self.read(oam_address),
             position_x: self.read(oam_address + 1),
            tile_number: self.read(oam_address + 2),
                  flags: OAMFlags {
                      priority: (flags & 0x80) >> 7,
                        flip_y: (flags & 0x40) >> 6,
                        flip_x: (flags & 0x20) >> 5,
                       palette: (flags & 0x10) >> 4
                  }
        }
    }

    // Helper

    fn addr_into_vram_space(address: u16) -> usize {
        address as usize - 0x8000
    }

    fn addr_into_oam_space(address: u16) -> usize {
        address as usize - 0xFE00
    }

    /// Returns the bg palette color in 00 ..= 11 associated with given 00 ..= 11
    fn bg_palette(&mut self, color: u8) -> u8 {

        let ff47 = self.read(0xFF47);

        match color {
            0b00 => ((ff47 & 0x01) << 1) | ((ff47 & 0x02) >> 1),    // Bits 0-1 -> 00
            0b01 => ((ff47 & 0x04) >> 1) | ((ff47 & 0x08) >> 3),    // Bits 2-3 -> 01
            0b10 => ((ff47 & 0x10) >> 3) | ((ff47 & 0x20) >> 5),    // Bits 4-5 -> 10
            0b11 => ((ff47 & 0x40) >> 5) | ((ff47 & 0x80) >> 7),    // Bits 6-7 -> 11

            _ => panic!("unexpected color code: {}", color)
        }
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