mod byte;
mod mode;
mod oam;

use byte::{
    lcdc::LCDC,
    lcds::LCDS
};

use mode::{Mode, Mode::*};
use oam::{OAMEntry, OAMFlags};

use crate::ppu::byte::Byte;

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
impl PPU {

    pub fn new() -> PPU {
        PPU {
            mode: Mode0,
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

            // VRAM Space
            0x8000 ..= 0x9FFF => match self.lcds.mode_flag()  {

                // Mode 0 / 1 / 2 allow VRAM access
                Mode0 | Mode1 | Mode2 => self.vram[PPU::addr_into_vram_space(address)],

                // Cannot access VRAM / OAM in Mode 3
                Mode3 => 0xFF,
            },

            // OAM Space
            0xFE00 ..= 0xFE9F => match self.lcds.mode_flag() {

                // Mode 0 / 1 allow OAM access
                Mode0 | Mode1 => self.oam[PPU::addr_into_oam_space(address)],

                // Cannot access OAM in Mode 2 / 3
                Mode2 | Mode3 => 0xFF
            },

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

            // VRAM Space
            0x8000 ..= 0x9FFF => match self.lcds.mode_flag()  {

                // Mode 0 / 1 / 2 allow VRAM access
                Mode0 | Mode1 | Mode2 => self.vram[PPU::addr_into_vram_space(address)] = value,

                // Cannot access VRAM / OAM in Mode 3
                Mode3 => (),
            },

            // OAM Space
            0xFE00 ..= 0xFE9F => match self.lcds.mode_flag() {

                // Mode 0 / 1 allow OAM access
                Mode0 | Mode1 => self.oam[PPU::addr_into_oam_space(address)] = value,

                // Cannot access OAM in Mode 2 / 3
                Mode2 | Mode3 => ()
            },

            // I/O Registers
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


    /// 'Main' Execution - run the PPU for a given number of cycles
    pub fn run_for(&mut self, cycles: u64) {

        let mut cycles_left = cycles;

        while cycles_left {

            // TODO

            cycles_left -= 1;
        }
    }

    fn draw_screen(&mut self) {


        // TODO - Flesh this out as we read

        // clear_screen ?

        // draw background ?

        // draw sprites ?
    }

    fn draw_row(&mut self) {

        let h = self.lcdc.obj_size().1;

        // 1 - OAM Search
        let mut visible: Vec<OAMEntry> = Vec::new();

        for entry_number in 0..=40 {

            let entry = self.oam_entry(entry_number);

            // Comparison method by which the GameBoy PPU uses to determine whether a given object
            //  should be considered 'visible' on the current line
            if entry.x != 0 && self.ly + 16 >= entry.y && self.ly + 16 < entry.y + h {
                visible.push(entry);
            }
        }

        // Order based on x position, as the 'first' (from the left) 10 sprites should be shown
        visible.sort_by(|a, b| b.x.cmp(&a.x));

        // GameBoy can only have up to 10 sprites per line, remove (don't draw) anything to the
        //  'right' of the first 10 sprites, as this is how the hardware will resolve >10 sprites
        visible.drain(10..);

        // 2 - Pixel Transfer

        // 3 - H-Blank
    }

    fn oam_entry(&mut self, entry_number: u8) -> OAMEntry {
        assert!(entry_number < 40, "asking for entry number beyond 40");

        // OAM entries are aligned on 4-byte boundaries beginning at 0xFE00
        let oam_address: u16 = 0xFE00 + (entry_number * 4) as u16;
        let flags = self.read(oam_address + 3);

        OAMEntry {
             y: self.read(oam_address),
             x: self.read(oam_address + 1),
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