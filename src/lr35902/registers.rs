use std::fmt;
use crate::lr35902::LR35902;

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

    /*************************/
    /*     Register Pairs    */
    /*************************/

    /***** AF *****/

    /// Get the register pair AF as a u16
    pub fn get_af(&self) -> u16 {
        LR35902::u16_from_u8(self.a, self.f)
    }

    /// Set the register pair AF to the given u16
    pub fn set_af(&mut self, x: u16) {
        let u8_pair = LR35902::u8_pair(x);
        self.a = u8_pair.0;
        self.f = u8_pair.1;
    }

    /***** BC *****/

    /// Get the register pair BC as a u16
    pub fn get_bc(&self) -> u16 {
        LR35902::u16_from_u8(self.b, self.c)
    }

    /// Set the register pair BC to the given u16
    pub fn set_bc(&mut self, x: u16) {
        let u8_pair = LR35902::u8_pair(x);
        self.b = u8_pair.0;
        self.c = u8_pair.1;
    }

    /***** DE *****/

    /// Get the register pair DE as a u16
    pub fn get_de(&self) -> u16 {
        LR35902::u16_from_u8(self.d, self.e)
    }

    /// Set the register pair DE to the given u16
    pub fn set_de(&mut self, x: u16) {
        let u8_pair = LR35902::u8_pair(x);
        self.d = u8_pair.0;
        self.e = u8_pair.1;
    }

    /***** HL *****/

    /// Get the register pair HL as a u16
    pub fn get_hl(&self) -> u16 {
        LR35902::u16_from_u8(self.h, self.l)
    }

    /// Set the register pair HL to the given u16
    pub fn set_hl(&mut self, x: u16) {
        let u8_pair = LR35902::u8_pair(x);
        self.h = u8_pair.0;
        self.l = u8_pair.1;
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


#[cfg(test)]
mod tests {

    use crate::lr35902::registers::Registers;

    #[test]
    fn get_initial() {
        let registers = Registers::new();
        assert_eq!(registers.a, 0x01);
        assert_eq!(registers.f, 0xB0);
        assert_eq!(registers.b, 0x00);
        assert_eq!(registers.c, 0x13);
        assert_eq!(registers.d, 0x00);
        assert_eq!(registers.e, 0xD8);
        assert_eq!(registers.h, 0x01);
        assert_eq!(registers.l, 0x4D);
        assert_eq!(registers.pc, 0x0100);
        assert_eq!(registers.sp, 0xFFFE);
        assert_eq!(registers.ime, false);
    }

    #[test]
    fn get_double() {
        let registers = Registers::new();
        assert_eq!(registers.get_af(), 0x01B0);
        assert_eq!(registers.get_bc(), 0x0013);
        assert_eq!(registers.get_de(), 0x00D8);
        assert_eq!(registers.get_hl(), 0x014D);
    }

    #[test]
    fn set_double() {
        let mut registers = Registers::new();

        registers.set_af(0xABCD);
        assert_eq!(registers.get_af(), 0xABCD);
        assert_eq!(registers.a, 0xAB);
        assert_eq!(registers.f, 0xCD);


        registers.set_bc(0xABCD);
        assert_eq!(registers.get_bc(), 0xABCD);
        assert_eq!(registers.b, 0xAB);
        assert_eq!(registers.c, 0xCD);

        registers.set_de(0xABCD);
        assert_eq!(registers.get_de(), 0xABCD);
        assert_eq!(registers.d, 0xAB);
        assert_eq!(registers.e, 0xCD);

        registers.set_hl(0xABCD);
        assert_eq!(registers.get_hl(), 0xABCD);
        assert_eq!(registers.h, 0xAB);
        assert_eq!(registers.l, 0xCD);
    }
}