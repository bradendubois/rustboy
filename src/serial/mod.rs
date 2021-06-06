use crate::traits::{MemoryMap, RunComponent};


const CPU_CYCLES_TO_SERIAL_RATIO: u64 = 512;


pub struct Serial {
    serial_transfer_data: u8,       // 0xFF01
    serial_transfer_control: u8,    // 0xFF02

    pub interrupt: bool,            // This corresponds to bit 3 of the IF register at 0xFF0F
    tick_tank: u64                  // A "tank" of cycles run to synchronize and run the Serial
                                    // transfer at an appropriate rate
}


impl Serial {

    pub fn new() -> Serial {
        Serial {
            serial_transfer_data: 0,
            serial_transfer_control: 0,
            interrupt: false,
            tick_tank: 0
        }
    }

    // TODO - Actual transfer method rotates bits along 0xFF01
}


impl RunComponent for Serial {

    fn run(&mut self, cpu_clock_cycles: u64) {
        self.tick_tank += cpu_clock_cycles;

        while self.tick_tank >= CPU_CYCLES_TO_SERIAL_RATIO {

            // TODO - actual transfer code here ...

            self.tick_tank -= CPU_CYCLES_TO_SERIAL_RATIO;
        }
    }
}


impl MemoryMap for Serial {

    fn read(&mut self, address: u16) -> u8 {
        match address {
            0xFF01 => self.serial_transfer_data,
            0xFF02 => self.serial_transfer_control,
            _ => panic!("serial link cable is not mapped to by address: {:#010X}", address)
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF01 => self.serial_transfer_data = value,
            0xFF02 => self.serial_transfer_control = value,
            _ => panic!("serial link cable is not mapped to by address: {:#010X}", address)
        }
    }
}