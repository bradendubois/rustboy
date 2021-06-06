use crate::traits::{MemoryMap, RunComponent};


const TIMER_TO_CPU_TICKS: usize = 256;


pub struct Timer {

    divider_register: u8,       // 0xFF04 - DIV
    timer_counter: u8,          // 0xFF05 - TIMA
    timer_modulo: u8,           // 0xFF06 - TMA
    timer_control: u8,          // 0xFF07 - TAC

    timer_enabled: bool,          // 0xFF07 - TAC - Timer Enable (Bit 2)
    timer_clock_select: usize,    // Determined by 0xFF07 - TAC - Input Clock  (Bits 1-0)

    divi_tank: usize,             // "Tank" of ticks to update the DIV Reg in appropriate intervals
    tima_tank: usize,             // "Tank" of ticks to update the TIMA Reg in appropriate intervals

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
}


impl MemoryMap for Timer {

    fn read(&mut self, address: u16) -> u8 {
        match address {
            0xFF04 => self.divider_register,
            0xFF05 => self.timer_control,
            0xFF06 => self.timer_modulo,
            0xFF07 => self.timer_control,

            _ => panic!("unmapped timer address: {:#06X}", address)
        }    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.divider_register = 0,
            0xFF05 => self.timer_control = value,
            0xFF06 => self.timer_modulo = value,
            0xFF07 => {
                self.timer_control = value & 0x07;
                self.timer_enabled = value & 0x04 != 0;
                self.timer_clock_select = match value & 0x3 {
                    0b00 => 1024,   // CPU Clock / 1024 =   4096 Hz = 1024 CPU ticks per Timer tick
                    0b01 => 16,     // CPU Clock / 16   = 262144 Hz =   16 CPU ticks per Timer tick
                    0b10 => 64,     // CPU Clock / 64   =  65536 Hz =   64 CPU ticks per Timer tick
                    0b11 => 256,    // CPU Clock / 256  =  16384 Hz =  256 CPU ticks per Timer tick

                    _ => panic!("impossible timer clock value: {:#010b}", value)
                };
            },

            _ => panic!("unmapped timer address: {:#06X}", address)
        }
    }
}


impl RunComponent for Timer {

    fn run(&mut self, cpu_clock_cycles: u64) {

        let cycles = cpu_clock_cycles as usize;

        // Divider timer always runs
        self.divi_tank += cycles;
        while self.divi_tank >= TIMER_TO_CPU_TICKS {
            self.divider_register = self.divider_register.wrapping_add(1);
            self.divi_tank -= TIMER_TO_CPU_TICKS;
        }

        // Only run timer counter if enabled
        if self.timer_enabled {

            self.tima_tank += cycles;

            // Conversion of 'tanked' CPU cycles into appropriate timer clock select cycles
            while self.tima_tank >= self.timer_clock_select {
                let (result, overflow) = self.timer_counter.overflowing_add(1);

                // Interrupt and reset on overflow
                self.timer_counter = match overflow {
                    true  => { self.interrupt = true; self.timer_modulo }
                    false => result
                };

                self.tima_tank -= self.timer_clock_select;
            }
        }
    }}