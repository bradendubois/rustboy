// use crate::ppu::PPU;

pub struct InterruptEnable {
    // ppu: PPU,
    // timer: Timer
    // serial: Serial
    // joypad: Joypad
}

impl InterruptEnable {

    pub fn new() -> InterruptEnable {
        InterruptEnable {
            // ppu
        }
    }

    pub fn read(&mut self) -> u8 {
        0
    }

    pub fn write(&mut self, value: u8) {
        let _value = value;
    }
}