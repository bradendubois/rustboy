use crate::ppu::PPU;

pub struct InterruptEnable {
    ppu: PPU,
    // timer: Timer
    // serial: Serial
    // joypad: Joypad
}

impl InterruptEnable {

    fn new(ppu: PPU) -> InterruptEnable {
        InterruptEnable {
            ppu
        }
    }
}