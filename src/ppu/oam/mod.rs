#[allow(dead_code)]
pub struct OAMFlags {
    pub priority: bool,   // OBJ-to-BG Priority   (0 = above BG,      1 = behind BG colors 1 - 3)
    pub   flip_y: bool,   // Y Flip               (0 = normal,        1 = vertical mirror)
    pub   flip_x: bool,   // X Flip               (0 = normal,        1 = horizontal mirror)
    pub  palette: bool    // Object Palette       (0 = obp0 @ 0xFF48, 1 = obp1 @ 0xFF49)
}

/// An entry in the OAM table for a sprite
#[allow(dead_code)]
pub struct OAMEntry {
    pub y: u8,              // Position X
    pub x: u8,              // Position Y
    pub tile_number: u8,    // Tile Number
    pub flags: OAMFlags     // Flags
}
