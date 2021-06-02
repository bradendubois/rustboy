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
            a: {:#010b} {:#04X} {} | f: {:#010b} {:#04X} {}\n\
            b: {:#010b} {:#04X} {} | c: {:#010b} {:#04X} {}\n\
            d: {:#010b} {:#04X} {} | e: {:#010b} {:#04X} {}\n\
            h: {:#010b} {:#04X} {} | l: {:#010b} {:#04X} {}\n\
            pc: {:#018b} {:#06X} {}\n\
            sp: {:#018b} {:#06X} {}",
               self.a, self.a, self.a, self.f, self.f, self.f,
               self.b, self.b, self.b, self.c, self.c, self.c,
               self.d, self.d, self.d, self.e, self.e, self.e,
               self.h, self.h, self.h, self.l, self.l, self.l,
               self.pc, self.pc,self.pc,
               self.sp, self.sp, self.sp)
    }
}
