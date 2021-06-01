use std::fmt;

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub f: u8,
    pub pc: u16,
    pub sp: u16,

    pub ime: bool,
}

impl Registers {

    pub fn new() -> Registers {
        Registers {
            a: 0x01, f: 0xB0,
            b: 0x00, c: 0x13,
            d: 0x00, e: 0xD8,
            h: 0x01, l: 0x4D,

            pc: 0x0100,
            sp: 0xFFFE,

            ime: false,
        }
    }
}


impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\
            a: {:#010b} f: {:#010b}\n\
            b: {:#010b} c: {:#010b}\n\
            d: {:#010b} e: {:#010b}\n\
            h: {:#010b} l: {:#010b}\n\
            pc: {:#018b}\n\
            sp: {:#018b}",
               self.a, self.f, self.b, self.c, self.d, self.e, self.h, self.l, self.pc, self.sp)
    }
}
