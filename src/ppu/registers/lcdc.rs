use crate::mmu::byte::Byte;

// Ranges for the window tile map
const WINDOW_TILE_MAP_DISPLAY_SELECT_0: (u16, u16) = (0x9800, 0x9BFF);
const WINDOW_TILE_MAP_DISPLAY_SELECT_1: (u16, u16) = (0x9C00, 0x9FFF);

// Ranges for the background and window tile data
const BG_WINDOW_TILE_DATA_SELECT_0: (u16, u16) = (0x8800, 0x97FF);
const BG_WINDOW_TILE_DATA_SELECT_1: (u16, u16) = (0x8000, 0x8FFF);

// Ranges for the background tile map
const BG_TILE_MAP_DISPLAY_SELECT_0: (u16, u16) = (0x9800, 0x9BFF);
const BG_TILE_MAP_DISPLAY_SELECT_1: (u16, u16) = (0x9C00, 0x9FFF);


// Valid Object (Sprite) dimensions
const OBJ_8_8: (u8, u8) = (8, 8);
const OBJ_8_16: (u8, u8) = (8, 16);



// Fields taken from: http://bgb.bircd.org/pandocs.htm#lcdcontrolregister
#[allow(dead_code)]
pub struct LCDC {

    /// LCD Display Enable
    /// Bit 7 : 0/False = Display Disabled, 1/True = Display Enabled
    lcd_display_enable: bool,

    /// Window Tile Map Display Select
    /// Bit 6 : 0/False = 0x9800-0x9BFF, 1/True = 0x9C00-0x9FFF
    window_tile_map_display_select: (u16, u16),

    /// Window Display Enable
    /// Bit 5 : 0/False = Window Disabled, 1/True = Window Enabled
    window_display_enable: bool,

    /// BG & Window Tile Data Select
    /// Bit 4 : 0 = 0x8800-0x97FF, 1 = 0x8000-0x8FFF
    bg_window_tile_data_select: (u16, u16),

    /// BG Tile Map Display Select
    /// Bit 3 : 0 = 0x9800-0x9BFF, 1 = 0x9C00-0x9FFF
    bg_tile_map_display_select: (u16, u16),

    /// Obj (Sprite) Size
    /// Bit 2 : 0 = 8x8, 1 = 8x16
    obj_size: (u8, u8),

    /// Obj (Sprite) Display Enable
    /// Bit 1 : 0/False = Objects Disabled, 1/True = Objects Enabled
    obj_display_enable: bool,

    /// BG Display
    /// Bit 0 : 0/False = BG Disabled, 1/True = BG Enabled
    bg_display: bool,

    // Real 8-bit value written
    value: u8
}


#[allow(dead_code)]
impl LCDC {

    pub fn new() -> LCDC {
        LCDC {
            lcd_display_enable: false,
            window_tile_map_display_select: WINDOW_TILE_MAP_DISPLAY_SELECT_0,
            window_display_enable: false,
            bg_window_tile_data_select: BG_WINDOW_TILE_DATA_SELECT_0,
            bg_tile_map_display_select: BG_TILE_MAP_DISPLAY_SELECT_0,
            obj_size: OBJ_8_8,
            obj_display_enable: false,
            bg_display: false,
            value: 0
        }
    }

    /// Check if the LCD is ON and PPU is active
    /// Returns true if the register bit is set and false otherwise
    pub fn lcd_display_enable(&self) -> bool { self.lcd_display_enable }

    /// Window tile map area
    /// if the bit is set the tile map area is 9C00-9FFF otherwise it's 9800-9BFF
    /// Controls the background map the window uses for rendering
    pub fn window_tile_map_display_select(&self) -> (u16, u16) { self.window_tile_map_display_select }

    /// Window enable
    /// Controls whether the window shall be displayed or not.
    pub fn window_display_enable(&self) -> bool { self.window_display_enable }

    /// BG and Window Tile Data Area
    /// controls which addressing mode the BG and Window use to pick tiles
    pub fn bg_window_tile_data_select(&self) -> (u16, u16) { self.bg_window_tile_data_select }

    /// BG Tile Map Area
    /// Similar to the window tile map area
    pub fn bg_tile_map_display_select(&self) -> (u16, u16) { self.bg_tile_map_display_select }

    /// OBJ Size
    /// Controls sprite size
    pub fn obj_size(&self) -> (u8, u8) { self.obj_size }

    /// OBJ Enable
    /// Toggles whether sprites are displayed or not
    pub fn obj_display_enable(&self) -> bool { self.obj_display_enable }

    /// BG and Window enable/priority
    /// This has different meanings depending on the gameboy type and mode
    /// [For more Info](https://gbdev.io/pandocs/LCDC.html#lcdc0---bg-and-window-enablepriority)
    pub fn bg_display(&self) -> bool { self.bg_display }

}

#[allow(dead_code)]
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
            false => WINDOW_TILE_MAP_DISPLAY_SELECT_0,
            true  => WINDOW_TILE_MAP_DISPLAY_SELECT_1
        };

        // Bit 5
        self.window_display_enable = value & 0x20 != 0;

        // Bit 4
        self.bg_window_tile_data_select = match value & 0x10 != 0 {
            false => BG_WINDOW_TILE_DATA_SELECT_0,
            true  => BG_WINDOW_TILE_DATA_SELECT_1
        };

        // Bit 3
        self.bg_tile_map_display_select = match value & 0x08 != 0 {
            false => BG_TILE_MAP_DISPLAY_SELECT_0,
            true  => BG_TILE_MAP_DISPLAY_SELECT_1
        };

        // Bit 2
        self.obj_size = match value & 0x04 != 0 {
            false => OBJ_8_8,
            true  => OBJ_8_16
        };

        // Bit 1
        self.obj_display_enable = value & 0x02 != 0;

        // Bit 0
        self.bg_display = value & 0x01 != 0;
    }
}