pub struct Serial {
    serial_transfer_data: u8,       // 0xFF01
    serial_transfer_control: u8     // 0xFF02
}

impl Serial {

    pub fn new() -> Serial {
        Serial {
            serial_transfer_data: 0,
            serial_transfer_control: 0
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xFF01 => self.serial_transfer_data,
            0xFF02 => self.serial_transfer_control,
            _ => panic!("serial link cable is not mapped to by address: {:#010X}", address)
        }
    }

    pub fn write(&mut self, value: u8, address: u16) {
        match address {
            0xFF01 => self.serial_transfer_data = value,
            0xFF02 => self.serial_transfer_control = value,
            _ => panic!("serial link cable is not mapped to by address: {:#010X}", address)
        }
    }

    // TODO - Actual transfer method rotates bits along 0xFF01
}
