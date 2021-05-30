// use crate::ppu::PPU;

pub struct InterruptFlag {
    // ppu: PPU,
    // timer: Timer
    // serial: Serial
    // joypad: Joypad
}

impl InterruptFlag {

    pub fn new() -> InterruptFlag {
        InterruptFlag {
        }
    }

    pub fn read(&mut self) -> u8 {
        // TODO - "build" the number from the various structs
        0
    }

    pub fn write(&mut self, value: u8) {
        let _value = value;
    }
}