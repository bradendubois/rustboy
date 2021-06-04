
#[allow(dead_code)]
pub struct Timer {
    divider_register: u8,       // 0xFF04 - DIV
    timer_counter: u8,          // 0xFF05 - TIMA
    timer_modulo: u8,           // 0xFF06 - TMA
    timer_control: u8,          // 0xFF07 - TAC
    timer_enabled: bool,        // 0xFF07 - TAC - Timer Enable (Bit 2)
    timer_clock_select: u32,    // Determined by 0xFF07 - TAC - Input Clock  (Bits 1-0)
    divi_tank: u32,             // "Tank" of ticks to update the DIV Reg in appropriate intervals
    tima_tank: u32,             // "Tank" of ticks to update the TIMA Reg in appropriate intervals

    pub interrupt: bool,    // This corresponds to bit 2 of the IF register at 0xFF0F
}

impl Timer {

    pub fn new() -> Timer {
        Timer {
            divider_register: 0,
            timer_counter: 0,
            timer_modulo: 0,
            timer_control: 0,
            timer_enabled: true,
            timer_clock_select: 0,
            divi_tank: 0,
            tima_tank: 0,
            interrupt: false,
        }
    }

    pub fn run(&mut self, ticks: u8) {

        self.divider_register = self.divider_register.wrapping_add(1);

        if self.timer_enabled {
            self.tima_tank += ticks as u32;
            while self.tima_tank >= self.timer_clock_select {
                let (result, overflow) = self.timer_counter.overflowing_add(1);

                if overflow {
                    self.timer_counter = self.timer_modulo;
                    self.interrupt = true;
                }

                self.timer_counter = result;
                self.tima_tank -= self.timer_clock_select;
            }
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        println!("ADDRESS: {:#06X}", address);
        match address {
            0xFF04 => self.divider_register,
            0xFF05 => self.timer_control,
            0xFF06 => self.timer_modulo,
            0xFF07 => self.timer_control,

            _ => panic!("unmapped timer address: {:#06X}", address)
        }
    }

    pub fn write(&mut self, value: u8, address: u16) {
        println!("ADDRESS: {:#06X} : {:#010b}", address, value);
        match address {
            0xFF04 => self.divider_register = 0,
            0xFF05 => self.timer_control = value,
            0xFF06 => self.timer_modulo = value,
            0xFF07 => {
                self.timer_control = value;
                self.timer_enabled = value & 0x04 != 0;
                self.timer_clock_select = match value & 0x03 {
                    0b00 => 16,
                    0b01 => 64,
                    0b10 => 256,
                    0b11 => 1024,

                    _ => panic!("impossible timer clock value: {:#010b}", value)
                };
            },

            _ => panic!("unmapped timer address: {:#06X}", address)
        }
    }
}