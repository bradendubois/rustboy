use crate::traits::MemoryMap;


pub struct Serial {
    serial_transfer_data: u8,       // 0xFF01
    serial_transfer_control: u8,    // 0xFF02

    pub interrupt: bool,            // This corresponds to bit 3 of the IF register at 0xFF0F
}


impl Serial {

    pub fn new() -> Serial {
        Serial {
            serial_transfer_data: 0,
            serial_transfer_control: 0,
            interrupt: false
        }
    }

    // TODO - Actual transfer method rotates bits along 0xFF01
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