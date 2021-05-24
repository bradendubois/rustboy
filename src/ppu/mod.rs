
enum Mode {
    Mode0,  // HBlank Period
    Mode1,  // VBlank Period
    Mode2   // Searching OAM Period
}


pub struct PPU {
    mode: Mode
}

impl PPU {

    pub fn new() -> PPU {
        PPU {
            mode: Mode::Mode0
        }
    }

    pub fn read_vram(&mut self, _address: u16) -> u8 {
        0
    }

    pub fn write_vram(&mut self, _value: u8, _address: u16) {

    }
}