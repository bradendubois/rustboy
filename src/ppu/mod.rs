
pub struct PPU {

}

impl PPU {

    pub fn new() -> PPU {
        PPU {

        }
    }

    pub fn read_vram(&mut self, _address: u16) -> u8 {
        0
    }

    pub fn write_vram(&mut self, _value: u8, _address: u16) {

    }
}