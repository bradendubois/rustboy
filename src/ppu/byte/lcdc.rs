use crate::ppu::byte::Byte;

// Fields taken from: http://bgb.bircd.org/pandocs.htm#lcdcontrolregister
pub struct LCDC {

    // Dissected 8-bit value interpretation
    lcd_display_enable: bool,
    window_tile_map_display_select: u16,
    window_display_enable: bool,
    bg_window_tile_data_select: u16,
    bg_tile_map_display_select: u16,
    obj_size: (u8, u8),
    obj_display_enable: bool,
    bg_display: bool,

    // Real 8-bit value written
    value: u8
}

impl Byte for LCDC {

    fn read(&self) -> u8 {
        self.value
    }

    /// Write given value into LCDC Register, populating ease-of-access fields
    fn write(&mut self, value: u8) {
        self.value = value;

        // Bit 7
        self.lcd_display_enable = value & 0x80 != 0;

        // Bit 6
        self.window_tile_map_display_select = match value & 0x40 != 0 {
            false => 0x9800,
            true  => 0x9C00
        };

        // Bit 5
        self.window_display_enable = value & 0x20 != 0;

        // Bit 4
        self.bg_window_tile_data_select = match value & 0x10 != 0 {
            false => 0x8800,
            true  => 0x8000
        };

        // Bit 3
        self.bg_tile_map_display_select = match value & 0x08 != 0 {
            false => 0x9800,
            true  => 0x9C00
        };

        // Bit 2
        self.obj_size = match value & 0x04 != 0 {
            false => (8,  8),
            true  => (8, 16)
        };

        // Bit 1
        self.obj_display_enable = value & 0x02 != 0;

        // Bit 0
        self.bg_display = value & 0x01 != 0;
    }
}