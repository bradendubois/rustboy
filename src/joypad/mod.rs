
#[allow(dead_code)]
pub struct Joypad {
    joypad: u8,             // 0xFF00 - P1/JOYP
    pub interrupt: bool     // This corresponds to bit 4 of the IF register at 0xFF0F
}

impl Joypad {
    pub fn new() -> Joypad {
        Joypad {
            joypad: 0,
            interrupt: false
        }
    }

    pub fn read(&self) -> u8 {
        self.joypad
    }

    pub fn write(&mut self, value: u8) {
        self.joypad = value & 0x30;     // Only care about about bits 5, 4, which correspond to 0x30
                                        // And correspond to P15, P14, respectively
    }

    // TODO sdl2 input to set bits for joypad
}