
#[allow(dead_code)]
pub struct Timer {
    divider_register: u8,   // 0xFF04 - DIV
    timer_counter: u8,      // 0xFF05 - TIMA
    timer_modulo: u8,       // 0xFF06 - TMA
    timer_control: u8       // 0xFF07 - TAC
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            divider_register: 0,
            timer_counter: 0,
            timer_modulo: 0,
            timer_control: 0
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        let _a = address;
        0
    }

    pub fn write(&mut self, value: u8, address: u16) {
        let _v = value;
        let _a = address;
    }
}