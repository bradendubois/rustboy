use crate::mmu::byte::Byte;

mod mode;
use mode::{Mode, Mode::*};

mod oam;
use oam::{OAMEntry, OAMFlags};

mod registers;
mod display;

use registers::lcdc::LCDC;
use registers::lcds::LCDS;
use std::ops::Range;
use sdl2::rect::Point;
use sdl2::pixels::Color;

const V_RAM_SIZE: usize = 0x2000;
const   OAM_SIZE: usize = 0x0100;

#[allow(dead_code)]
const  TILE_SIZE: usize = 128;

/// GameBoy Screen Height
#[allow(dead_code)]
pub const HEIGHT: u8 = 144;
pub const MAX_SCREEN_Y: u8 = HEIGHT - 1;

/// GameBoy Screen Width
#[allow(dead_code)]
pub const WIDTH: usize = 160;


const   CYCLES_PER_LINE: u8 = 114;
// const OAM_SEARCH_CYCLES: u64 = 20;
const     V_BLANK_LINES: u8 = 10;

const TOTAL_LINES: u8 = HEIGHT + V_BLANK_LINES;


#[allow(dead_code)]
pub struct PPU {

    display: display::Screen,

    clock: u64,     // Behaves as a counter of how many cycles / ticks have occurred, used to determine
                    // an appropriate "mode" to switch to at a given point

    mode: Mode,                 // PPU Mode
    vram: [u8; V_RAM_SIZE],     // VRAM
     oam: [u8; OAM_SIZE],       // OAM / Sprite Attribute Table

    lcdc: LCDC,     // 0xFF40 : LCDC Register : LCD C(ontrol) Register
    lcds: LCDS,     // 0xFF41 : LCDS Register : LCD S(tatus) Register
     scy: u8,       // 0xFF42 : Scroll Y
     scx: u8,       // 0xFF43 : Scroll X
      ly: u8,       // 0xFF44 : LY  (LCD Y)
     lyc: u8,       // 0xFF45 : LYC (LY Compare)
     bgp: u8,       // 0xFF47 : BGP Palette Data (Non-CGB)
    obp0: u8,       // 0xFF48 : Object Palette 0 (Non-CGB)
    obp1: u8,       // 0xFF49 : Object Palette 1 (Non-CGB)
      wy: u8,       // 0xFF4A : Window Y Position
      wx: u8,       // 0xFF4B : Window X Position

    pub stat_interrupt: bool,       // A flag to represent a STAT interrupt request; this
                                    // corresponds to bit 1 in the 0xFF0F (Interrupt Flag) register

    pub vblank_interrupt: bool      // A flag to represent a VBlank interrupt request; this flag
                                    // corresponds to bit 0 in the 0xFF0F (Interrupt Flag) register
}


#[allow(dead_code)]
impl PPU {

    pub fn new() -> PPU {
        PPU {
            clock: 0,
            display: display::Screen::new(),

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
              wx: 0,

              stat_interrupt: false,
            vblank_interrupt: false
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
            0xFF46 => panic!("cannot read DMA register"),
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,

            _ => panic!("unmapped address: {}", address)
        }
    }

    pub fn write(&mut self, value: u8, address: u16) {

        // println!("address: {:#06X}", address);

        if address == 0xFF00 || address == 0xFF01 {
            println!("address: {:#06X} {:010b}", address, value);
        }

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
            0xFF46 => self.dma_transfer(value),
            0xFF47 => self.bgp = value,
            0xFF48 => self.obp0 = value,
            0xFF49 => self.obp1 = value,
            0xFF4A => self.wy = value,
            0xFF4B => self.wx = value,

            _ => panic!("unmapped address: {}", address)
        }
    }


    pub fn dma_transfer(&mut self, value: u8) {

        return;

        let source = ((value as u16) << 8) * 0x0100;

        for i in 0x00..=0x9F {
            let read = self.read(source + i as u16);
            self.write(read, 0xFE00 + i as u16);
        }

        self.clock += 80;
    }


    /// 'Main' Execution - run the PPU for a given number of cycles
    pub fn run(&mut self, cycles: u64) {

        // Display is turned off
        if !self.lcdc.lcd_display_enable() {
            return
        }

        let mut cycles_left = cycles;

        while cycles_left > 0 {

            let current = std::cmp::min(cycles, 80);

            // Any amount of "cycles given" should be cleanly divisible by 4, as the CPU will call
            //  this function, and should 'scale' the cycles by 4, to correspond with the difference
            //  between the CPU and PPU in clock speed
            assert_eq!(current % 4, 0);
            self.clock += current / 4;

            // One horizontal row is completed, as one horizontal row takes 114 cycles
            //  1 row = 20 cycles (OAM Search) + ~43 cycles (Pixel Transfer) + ~51 cycles (H-Blank)
            if self.clock >= CYCLES_PER_LINE as u64 {
                self.clock %= CYCLES_PER_LINE as u64;

                // Advance to next line (wrapping "around" back to top if necessary)
                self.ly += 1;
                self.ly %= HEIGHT + V_BLANK_LINES;

                // If toggled, check for lyc interrupt
                if self.lcds.lyc_interrupt() && self.ly == self.lyc {

                    // Enable the LCDS Coincidence bit
                    let value = self.lcds.read() | 0x04;
                    self.lcds.write(value);

                    self.stat_interrupt = true;
                }
            }


            // Compute the mode the PPU 'should' be in, based on whether ly is on the visible area,
            //  (and if so, whether in OAM / Transfer / HBlank), or in a VBlank area
            let target_mode = match self.ly {

                0 ..= MAX_SCREEN_Y => match self.clock {
                     0 ..= 20 => Mode2,
                    21 ..= 43 => Mode3,
                    44 ..= 114 => Mode0,

                    _ => panic!("impossible clock value: {}", self.clock)
                },

                HEIGHT ..= TOTAL_LINES => Mode1,

                _ => panic!("impossible ly value: {}", self.ly)
            };

            // Check before setting the mode, as interrupts can be generated by switching modes
            if self.mode != target_mode {
                self.enter_mode(target_mode);
            }

            cycles_left = match cycles_left.checked_sub(current) {
                Some(i) => i,
                None => 0
            };
        }
    }

    fn enter_mode(&mut self, mode: Mode) {

        // Compute any byproducts (such as interrupts or line draws) from this mode switch
        match mode {

            // Entering HBlank Mode
            Mode0 => {

                // Entering HBlank mode indicates pixel transfer is complete, and so the row
                //  can be drawn
                self.draw_row();

                if self.lcds.mode_0_h_blank_interrupt() {
                    self.stat_interrupt = true;
                }
            }

            // Entering VBlank Mode
            Mode1 => {
                self.vblank_interrupt = true;
                if self.lcds.mode_1_v_blank_interrupt() {
                    self.stat_interrupt = true;
                }
            }

            // Entering OAM Search mode
            Mode2 => if self.lcds.mode_2_oam_interrupt() { self.stat_interrupt = true; }

            // No interrupts / changes from Pixel Transfer mode
            Mode3 => ()
        };

        self.mode = mode;
    }

    fn draw_row(&mut self) {

        let h = self.lcdc.obj_size().1;

        // 1 - OAM Search
        let mut visible: Vec<OAMEntry> = Vec::new();

        for entry_number in 0..40 {

            let entry = self.oam_entry(entry_number);

            // Comparison method by which the GameBoy PPU uses to determine whether a given object
            //  should be considered 'visible' on the current line
            if entry.x != 0 && self.ly + 16 >= entry.y && self.ly + 16 < entry.y + h {
                visible.push(entry);
            }
        }

        // Order based on x position, as the 'first' (from the left) 10 sprites should be shown
        // TODO - Perhaps it's really just the first 10 from the start of the OAM, no sort?
        visible.sort_by(|a, b| b.x.cmp(&a.x));

        // GameBoy can only have up to 10 sprites per line, remove (don't draw) anything to the
        //  'right' of the first 10 sprites, as this is how the hardware will resolve >10 sprites
        if visible.len() > 10 {
            visible.drain(10..);
        }

        // 2 - Pixel Transfer

        // 2.a - Clear Background

        let mut clear_pixels = Vec::new();
        for i in 0..WIDTH {
            clear_pixels.push((Point::new(i as u32 as i32, self.ly as u32 as i32), Color::RGB(255, 255, 255)));
        }
        // self.display.draw(clear_pixels);

        // let clear_bg = (0..WIDTH).into_iter().map(|x| (Point::new(x as u32 as i32, self.ly as u32 as i32), Color::RGB(0, 0, 0))).collect();
        //self.display.draw(clear_bg);

        // 2.b - Background
        // let background_pixels = self.background_pixels();
        // self.display.draw(background_pixels);

        // 2.c - Objects
        // let object_pixels = self.object_pixels(visible);
        // self.display.draw(object_pixels);


        let mut dump = Vec::new();
        let mut base = 0x8000;

        for y in 0..18 {
            for x in 0..20 {

                let start = 0x8000 as u16 + (y * 16 * 18) + (x * 16);
                let dx = (x * 8) as u32 as i32 - 8;
                let dy = (y * 8) as u32 as i32 - 16;

                for oy in 0..8 {

                    //println!("{:#06X} {:#06X}", start + (oy * 2), start + (oy * 2) + 1);

                    let txt1 = self.read(start + (oy * 2));
                    let txt2 = self.read(start + (oy * 2) + 1);

                    //println!("{:#010b} {:#010b}", txt1, txt2);

                    for ox in 0..8 {

                        let v0 = if (txt1 & (0x80 >> ox)) != 0 { 1 } else { 0 };
                        let v1 = if (txt2 & (0x80 >> ox)) != 0 { 1 } else { 0 };

                        let v = v0 << 1 | v1;

                        if v == 0 { continue }

                        let color = match v {
                            0b00 => Color::RGB(0, 0, 0),
                            0b01 => Color::RGB(60, 60, 60),
                            0b10 => Color::RGB(120, 120, 120),
                            0b11 => Color::RGB(180, 180, 180),
                            _ => panic!("{}", v)
                        };

                        dump.push((Point::new(dx + ox as u32 as i32,  dy + oy as u32 as i32), color));

                    }
                }
            }
        }

        /*
        if dump.len() > 0 {
            println!("{:?}", dump);
        }

         */

        // self.display.draw(dump);

        // 3 - H-Blank
        // self.display.draw(dump);
    }


    fn object_pixels(&mut self, visible: Vec<OAMEntry>) -> Vec<(Point, Color)> {

        let mut pixels = Vec::new();

        return pixels;

        if visible.len() > 0 {
            println!("******************************** active sprites");
        }

        for object in visible.iter() {

            let destination_x = ((object.x as i8) - 8) as i32;
            let destination_y = ((object.y as i8) - 16) as i32;

            let obj_y = match object.flags.flip_y {
                true  => self.lcdc.obj_size().1 - 1 - (self.ly - object.y),
                false => self.ly - object.y
            };

            assert!(obj_y >= 0 && obj_y < 16);

            for x in 0..8 {

                if object.x + x < 0 || object.x + x >= WIDTH as u8 {
                    continue;
                }

                let col = match object.flags.flip_x {
                    true => 7 - x,
                    false => x
                };

                let address = 0x8000 as u16 + (object.tile_number * 16) as u16 + (obj_y * 2) as u16;

                let object_bytes = (self.read(address), self.read(address + 1));

                let color = (object_bytes.0 & (1 << col)) << 1 | (object_bytes.1 & (1 << col));

                // Translucent
                if color == 0b00 {
                    continue;
                }

                // TODO - Connect to "real" palette
                let color = match color {

                    0b01 => Color::RGB(80, 80, 80),
                    0b10 => Color::RGB(140, 140, 140),
                    0b11 => Color::RGB(200, 200, 200),

                    _ => panic!("impossible color: {}", color)
                };

                pixels.push((Point::new(destination_x, destination_y), color));
            }
        }

        pixels
    }

    fn background_pixels(&mut self) -> Vec<(Point, Color)> {

        let mut pixels: Vec<(Point, Color)> = Vec::new();

        if !self.lcdc.bg_display() {
            return pixels
        }

        let y = self.ly as i32 - self.wy as i32;

        if y < 0 {
            return pixels
        }

        /*
        let sprite_bytes = match self.lcdc.bg_window_tile_data_select().0 {

            // "8000" method
            0x8000 => {

                let base_tile_address = object.tile_number as u16;
                let address = base_tile_address * 16 + (object_data_y * 2) as u16;
                let address = 0x8000 + address as u16;

                (self.read(address),  self.read(address + 1))
            },

            // "8800" method
            0x8800 => {

                let base_tile_address = (object.tile_number * 16) as i16;
                let address = (0x9000 as i32 + base_tile_address as i32) as u16 + (object_data_y * 2) as u16;

                (self.read(address), self.read(address + 1))
            },

            _ => panic!("impossible addressing method: {:?}", self.lcdc.bg_window_tile_data_select())
        };

         */

        pixels

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
                      priority: (flags & 0x80) >> 7 != 0,
                        flip_y: (flags & 0x40) >> 6 != 0,
                        flip_x: (flags & 0x20) >> 5 != 0,
                       palette: (flags & 0x10) >> 4 != 0
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
