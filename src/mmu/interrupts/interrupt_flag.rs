use crate::ppu::PPU;
use crate::mmu::byte::Byte;

pub struct InterruptFlag {
    ppu: PPU,
    // timer: Timer
    // serial: Serial
    // joypad: Joypad
}

impl InterruptFlag {

    fn new(ppu: PPU) -> InterruptFlag {
        InterruptFlag {
            ppu
        }
    }

    fn read(&mut self) -> u8 {
        // TODO - "build" the number from the various structs
        self.ppu.
    }

    fn write(&mut self, value: u8) {

    }
}