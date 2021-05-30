use crate::ppu::byte::Byte;
use crate::ppu::Mode;

// Fields taken from: http://bgb.bircd.org/pandocs.htm#lcdstatusregister
pub struct LCDS {

    // Dissected 8-bit value interpretation
    lyc_interrupt: bool,
    mode_2_oam_interrupt: bool,
    mode_1_v_blank_interrupt: bool,
    mode_0_h_blank_interrupt: bool,
    coincidence_flag: bool,
    mode_flag: Mode,

    // Real 8-bit value written
    value: u8
}

impl LCDS {
    pub fn new() -> LCDS {
        LCDS {
            lyc_interrupt: false,
            mode_2_oam_interrupt: false,
            mode_1_v_blank_interrupt: false,
            mode_0_h_blank_interrupt: false,
            coincidence_flag: false,
            mode_flag: Mode::Mode0,
            value: 0
        }
    }
}

impl Byte for LCDS {

    fn read(&self) -> u8 {
        self.value
    }

    /// Write given value into LCDS Register, populating ease-of-access fields
    fn write(&mut self, value: u8) {

        self.value = value;

        // Bit 7
        // self.lcd_display_enable = value & 0x80 != 0;

        // Bit 6
        /*
        self.window_tile_map_display_select = match value & 0x40 != 0 {
            false => 0x9800,
            true  => 0x9C00
        };

         */

        // Bit 5
        // self.window_display_enable = value & 0x20 != 0;

        // Bit 4
        /*
        self.bg_window_tile_data_select = match value & 0x10 != 0 {
            false => 0x8800,
            true  => 0x8000
        };

         */

        // Bit 3
        /*
        self.bg_tile_map_display_select = match value & 0x08 != 0 {
            false => 0x9800,
            true  => 0x9C00
        };
         */

        // Bit 2
        /*
        self.obj_size = match value & 0x04 != 0 {
            false => (8,  8),
            true  => (8, 16)
        };

         */

        // Bit 1
        // self.obj_display_enable = value & 0x02 != 0;

        // Bit 0
        // self.bg_display = value & 0x01 != 0;
    }
}