use super::mmu;
use super::instructions;

#[derive(Debug)]
pub struct Clock {
    pub m: u16,
    pub t: u16
}


#[allow(dead_code)]
#[derive(Debug)]
pub enum Status{
    STOPPED,
    HALTED,
    RUNNING
}


#[derive(Debug)]
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
}

// Struct representing the Z80 CPU
#[derive(Debug)]
pub struct Z80 {

    // Struct of all registers in the Z80
    pub registers: Registers,

    // Struct representing the clock of the Z80 for purposes of timing
    pub clock: u64,

    // Enum representing the Z80's current running status
    pub status: Status,

    // Struct representing the memory unit
    pub mmu: mmu::MMU
}



#[allow(dead_code)]
impl Z80 {

    // Initialization / creation of a Z80 CPU
    pub fn new(mmu: mmu::MMU) -> Z80 {

        Z80 {

            // All registers begin empty
            registers: Registers {
                a: 0x01,
                f: 0xB0,

                b: 0x00,
                c: 0x13,

                d: 0x00,
                e: 0xD8,

                h: 0x01,
                l: 0x4D,

                pc: 0x0100,
                sp: 0xFFFE,
            },

            // Clock begins at 0
            clock: 0,

            // status enum starts as running.
            status: Status::RUNNING,

            // MMU Unit
            mmu
        }
    }

    // Basic execution of the current operation at the program-counter (PC) register
    pub fn step(&mut self) {

        // Get the opcode number to execute
        let opcode = self.byte();

        // Fetch the opcode
        let opcode = instructions::Opcode::lookup(opcode);

        // Execute
        let cycles = (opcode.instruction)(self);

        // Adjust clock and program counter (PC)
        self.clock += cycles as u64;
    }

    pub fn byte(&mut self) -> u8 {
        let next_byte = self.mmu.read(self.registers.pc);
        self.registers.pc += 1;
        next_byte
    }

    pub fn word(&mut self) -> u16 {
        let lower = self.byte();
        let upper = self.byte();
        Z80::u16_from_u8(upper, lower)
    }

    // Set CPU F flags

    // Zero

    pub fn set_zero(&mut self) {
        self.registers.f |= 0x80;
    }

    pub fn unset_zero(&mut self) {
        self.registers.f ^= 0x80;
    }

    pub fn is_zero(&self) -> bool {
        self.registers.f & 0x80 != 0
    }

    // Subtraction

    pub fn set_subtraction(&mut self) {
        self.registers.f |= 0x40;
    }

    pub fn unset_subtraction(&mut self) {
        self.registers.f ^= 0x40;
    }

    pub fn is_subtraction(&self) -> bool {
        self.registers.f & 0x40 != 0
    }

    // Half Carry

    pub fn set_half_carry(&mut self) {
        self.registers.f |= 0x20;
    }

    pub fn unset_half_carry(&mut self) {
        self.registers.f ^= 0x20;
    }

    pub fn is_half_carry(&self) -> bool {
        self.registers.f & 0x20 != 0
    }

    // Full Carry

    pub fn set_full_carry(&mut self) {
        self.registers.f |= 0x10;
    }

    pub fn unset_full_carry(&mut self) {
        self.registers.f ^= 0x10;
    }

    pub fn is_full_carry(&self) -> bool {
        self.registers.f & 0x10 != 0
    }

    // Context specific flag methods - give parameters to see whether flags should be set

    fn zero(&mut self, result: u16) {
        match result {
            0 => self.set_zero(),
            _ => self.unset_zero()
        };
    }

    fn half_carry(&mut self, before: u8, after: u8) -> bool {
        (after >> 4) > (before >> 4)
    }

    /*************************/
    /*          ALU          */
    /*************************/

    pub fn daa(&mut self) {

        let mut a = self.registers.a;
        let mut adj = 0x00;

        if self.is_full_carry() { adj |= 0x60; }
        if self.is_half_carry() { adj |= 0x06; }

        if !self.is_subtraction() {
            if a & 0x0F > 0x09 { adj |= 0x06; };
            if a > 0x99 { adj |= 0x60; };
        }

        a = a.wrapping_add(adj);

        match adj >= 0x60 {
            true => self.set_full_carry(),
            false => self.unset_full_carry()
        };

        self.unset_half_carry();
        self.zero(a.into());

        self.registers.a = a;
    }

    // Add

    pub fn add_8(&mut self, s: u8, t: u8, flag: bool) -> u8 {

        let result = s.wrapping_add(t);

        if flag {

            match result {
                0 => self.set_zero(),
                _ => self.unset_zero()
            };

            self.unset_subtraction();

            match ((s & 0xF) + (t & 0xF)) > 0xF {
                true => self.set_half_carry(),
                false => self.unset_half_carry()
            };

            match s.checked_add(t) {
                None => self.set_full_carry(),
                Some(_) => self.unset_full_carry()
            };
        }

        result
    }

    pub fn add_16(&mut self, s: u16, t: u16, flag: bool) -> u16 {

        let result = s.wrapping_add(t);

        if flag {
            self.unset_subtraction();

            match (s & 0x07FF) + (t & 0x07FF) > 0x07FF  {
                true => self.set_half_carry(),
                false => self.unset_half_carry()
            };

            match s.checked_add(t) {
                None => self.set_full_carry(),
                Some(_) => self.unset_full_carry()
            };
        }

        result
    }

    // Sub

    pub fn sub_8(&mut self, s: u8, t: u8, flag: bool) -> u8 {

        let result = s.wrapping_sub(t);

        if flag {
            self.zero(result as u16);
        }

        result
    }

    pub fn sub_16(&mut self, s: u16, t: u16, flag: bool) -> u16 {

        let result = s.wrapping_sub(t);

        if flag {
            self.zero(result as u16);

        }

        result
    }

    // Inc

    pub fn inc_8(&mut self, s: u8, flag: bool) -> u8 {

        let result = s.wrapping_add(1);

        if flag {
            self.zero(result as u16);
            self.unset_subtraction();

            match (s & 0xF) + (s+1 & 0xF) > 0xF  {
                true => self.set_half_carry(),
                false => self.unset_half_carry()
            };
        }

        result
    }

    pub fn inc_16(&mut self, s: u16, flag: bool) -> u16 {

        let result = s.wrapping_add(1);

        if flag {
            self.zero(result);
            self.unset_subtraction();

            match (s & 0x07FF) + (s+1 & 0x07FF) > 0x07FF  {
                true => self.set_half_carry(),
                false => self.unset_half_carry()
            };
        }

        result
    }

    // Dec

    pub fn dec_8(&mut self, s: u8, flag: bool) -> u8 {

        let result = s.wrapping_sub(1);

        if flag {

        }

        result
    }

    pub fn dec_16(&mut self, s: u16, flag: bool) -> u16 {

        let result = s.wrapping_sub(1);

        if flag {

        }

        result
    }

    // ADC

    pub fn adc_8(&mut self, s: u8, t: u8) -> u8 {

        let carry = match self.is_full_carry() {
            true => 1,
            false => 0
        };

        self.add_8(s, t + carry, true)
    }

    // AND

    pub fn and(&mut self, t: u8) {

        let result = self.registers.a & t;

        match result == 0 {
            true => self.set_zero(),
            false => self.unset_zero()
        };

        self.unset_subtraction();
        self.unset_half_carry();
        self.set_full_carry();

        self.registers.a = result;
    }

    // XOR

    pub fn xor(&mut self, t: u8) {

        let result = self.registers.a ^ t;

        match result == 0 {
            true => self.set_zero(),
            false => self.unset_zero()
        };

        self.unset_subtraction();
        self.unset_half_carry();
        self.unset_full_carry();
    }

    // Conversions

    pub fn u16_from_u8(x: u8, y: u8) -> u16 {
        ((x << 8) + y).into()
    }

    pub fn u8_pair(x: u16) -> (u8, u8) {
        ((x >> 8) as u8, x as u8)
    }

    /*************************/
    /*     Register Pairs    */
    /*************************/

    // AF
    pub fn get_af(&self) -> u16 {
        Z80::u16_from_u8(self.registers.a, self.registers.f)
    }

    pub fn set_af(&mut self, x: u16) {
        let u8_pair = Z80::u8_pair(x);
        self.registers.a = u8_pair.0;
        self.registers.f = u8_pair.1;
    }

    // BC
    pub fn get_bc(&self) -> u16 {
        Z80::u16_from_u8(self.registers.b, self.registers.c)
    }

    pub fn set_bc(&mut self, x: u16) {
        let u8_pair = Z80::u8_pair(x);
        self.registers.b = u8_pair.0;
        self.registers.c = u8_pair.1;
    }

    // DE
    pub fn get_de(&self) -> u16 {
        Z80::u16_from_u8(self.registers.d, self.registers.e)
    }

    pub fn set_de(&mut self, x: u16) {
        let u8_pair = Z80::u8_pair(x);
        self.registers.d = u8_pair.0;
        self.registers.e = u8_pair.1;
    }

    // HL
    pub fn get_hl(&self) -> u16 {
        Z80::u16_from_u8(self.registers.h, self.registers.l)
    }

    pub fn set_hl(&mut self, x: u16) {
        let u8_pair = Z80::u8_pair(x);
        self.registers.h = u8_pair.0;
        self.registers.l = u8_pair.1;
    }
}
