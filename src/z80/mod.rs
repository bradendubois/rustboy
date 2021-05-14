use super::mmu;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Status {
    STOPPED,
    HALTED,
    RUNNING,
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

    pub ime: bool,
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

    // CB Flag : Will set whether to use the default table or the CB Prefix table
    pub use_cb_table: bool,

    // Struct representing the memory unit
    pub mmu: mmu::MMU,
}

#[allow(dead_code)]
impl Z80 {
    /// Initializer for a Z80 CPU
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

                ime: false,
            },

            // Clock begins at 0
            clock: 0,

            // status enum starts as running.
            status: Status::RUNNING,

            use_cb_table: false,

            // MMU Unit
            mmu,
        }
    }

    /// Run the CPU, fetching/decoding/executing at the PC until otherwise halted / interrupted
    pub fn step(&mut self) {
        loop {
            // Get the opcode number to execute
            let opcode = self.byte();

            // Execute from standard table
            let cycles = self.call_instruction(opcode);

            // Adjust clock and program counter (PC)
            self.clock += cycles as u64;
        }
    }

    /*************************/
    /*          ALU          */
    /*************************/

    /*        Addition       */

    /// Add two u8s together, handling overflow and the Z/N/H/C flags of the F register
    pub fn add_8(&mut self, s: u8, t: u8) -> u8 {
        let result = s.wrapping_add(t);

        match result {
            0 => self.set_zero(),
            _ => self.unset_zero(),
        };

        self.unset_subtraction();

        match ((s & 0xF) + (t & 0xF)) > 0xF {
            true => self.set_half_carry(),
            false => self.unset_half_carry(),
        };

        match s.checked_add(t) {
            None => self.set_full_carry(),
            Some(_) => self.unset_full_carry(),
        };

        result
    }

    /// Add two u16s together, handling overflow and the Z/N/H/C flags of the F register
    pub fn add_16(&mut self, s: u16, t: u16) -> u16 {
        let result = s.wrapping_add(t);

        self.unset_subtraction();

        match (s & 0x07FF) + (t & 0x07FF) > 0x07FF {
            true => self.set_half_carry(),
            false => self.unset_half_carry(),
        };

        match s.checked_add(t) {
            None => self.set_full_carry(),
            Some(_) => self.unset_full_carry(),
        };

        result
    }

    /// ADC - Add the given value and the carry (C) flag to the accumulator (A) register
    pub fn adc_8(&mut self, s: u8) {
        let carry = match self.is_full_carry() {
            true => 1,
            false => 0,
        };

        self.registers.a = self.add_8(self.registers.a, s + carry);
    }

    /// SBC - Subtract given value and carry flag from the A register
    pub fn sbc_8(&mut self, s: u8) {
        let carry = match self.is_full_carry() {
            true => 1,
            false => 0,
        };
        self.registers.a = self.sub_8(self.registers.a, s + carry);
    }

    /*      Subtraction      */

    /// Subtract t (u8) from s (u8), handling underflow and the Z/N/H/C flags of the F register
    pub fn sub_8(&mut self, s: u8, t: u8) -> u8 {
        let result = s.wrapping_sub(t);

        match result == 0 {
            true => self.set_zero(),
            false => self.unset_zero(),
        };

        self.set_subtraction();
        match ((s & 0xf) - (t & 0xf)) & 0x10 != 0 {
            true => self.set_half_carry(),
            false => self.unset_half_carry(),
        };

        result
    }

    /*     Incrementation    */

    /// Increment a given u8, handling overflow and the Z/N/H/C flags of the F register
    pub fn inc_8(&mut self, s: u8) -> u8 {
        // Save the carry flag as it is changed by sub
        let carry = self.is_full_carry();
        let result = self.add_8(s, 1);

        // Restore the carry flag state after sub operation
        match carry {
            true => self.set_full_carry(),
            false => self.unset_full_carry(),
        };

        result
    }

    /// Increment a given u16, handling overflow
    pub fn inc_16(&mut self, s: u16) -> u16 {

        s.wrapping_add(1)

        // Save the carry flag as it is changed by sub
        let carry = self.is_full_carry();
        let result = self.add_16(s, 1);

        // Restore the carry flag state after sub operation
        match carry {
            true => self.set_full_carry(),
            false => self.unset_full_carry(),
        };

        result

    }

    /*      Decrementing     */

    /// Decrement a given u8, handling overflow and the Z/N/H/C flags of the F register
    pub fn dec_8(&mut self, s: u8) -> u8 {
        // Save the carry flag as it is changed by sub
        let carry = self.is_full_carry();
        let result = self.sub_8(s, 1);

        // Restore the carry flag state after sub operation
        match carry {
            true => self.set_full_carry(),
            false => self.unset_full_carry(),
        };

        result
    }

    /// Decrement a given u16, handling overflow and the Z/N/H/C flags of the F register
    pub fn dec_16(&mut self, s: u16) -> u16 {
        // Save the carry flag as it is changed by sub
        let carry = self.is_full_carry();
        let result = s.wrapping_sub(1);

        // Restore the carry flag state after sub operation
        match carry {
            true => self.set_full_carry(),
            false => self.unset_full_carry(),
        };

        result
    }

    /*   Bitwise Operations  */

    /// AND - AND the given value with the accumulator register (A) and store the result in A
    pub fn and(&mut self, t: u8) {
        self.registers.a &= t;

        match self.registers.a == 0 {
            true => self.set_zero(),
            false => self.unset_zero(),
        };

        self.unset_subtraction();
        self.unset_half_carry();
        self.set_full_carry();
    }

    /// OR - OR the given value with register A. Store result in A.
    pub fn or(&mut self, t: u8){
        self.registers.a |= t;

        match self.registers.a == 0{
            true => self.set_zero(),
            false => self.unset_zero(),
        };
        self.unset_subtraction();
        self.unset_half_carry();
        self.unset_full_carry();
    }

    /// CP - Compare the given value with register A, setting the zero flag if they're equal
    pub fn cp(&mut self, t: u8){
        if self.sub_8(self.registers.a,t) == 0{
            self.set_zero()
        }
    }

    /// XOR - XOR the given value with the accumulator register (A) and store the result in A
    pub fn xor(&mut self, v: u8) {
        self.registers.a ^= v;

        match self.registers.a == 0 {
            true => self.set_zero(),
            false => self.unset_zero(),
        };

        self.unset_subtraction();
        self.unset_half_carry();
        self.unset_full_carry();
    }

    /// RLC - Rotate a number left, and copy the left-most bit shifted into the C register
    pub fn rlc(&mut self, v: u8) -> u8 {

        let result = (v << 1) | (v >> 7);

        match result == 0 {
            true => self.set_zero(),
            false => self.unset_zero()
        };

        self.unset_subtraction();
        self.unset_half_carry();

        match result & 0x01 == 0 {
            true => self.unset_full_carry(),
            false => self.set_full_carry()
        };

        result
    }

    /// RRC - Rotate a number right, and copy the right-most bit shifted into the C register
    pub fn rrc(&mut self, v: u8) -> u8 {

        let result = (v >> 1) | (v << 7);

        match result == 0 {
            true => self.set_zero(),
            false => self.unset_zero()
        };

        self.unset_subtraction();
        self.unset_half_carry();

        match result & 0xF0 == 0 {
            true => self.unset_full_carry(),
            false => self.set_full_carry()
        };

        result
    }

    /// SLA - Shift a number left, and copy the left-most bit shifted into the C register
    pub fn sla(&mut self, v: u8) -> u8 {

        let result = v << 1;

        match result == 0 {
            true => self.set_zero(),
            false => self.unset_zero()
        };

        self.unset_subtraction();
        self.unset_half_carry();

        match v & 0xF0 == 0 {
            true => self.unset_full_carry(),
            false => self.set_full_carry()
        };

        result
    }

    /// SRA - Shift a number right, and copy the right-most bit shifted into the C register
    pub fn sra(&mut self, v: u8) -> u8 {

        let result = v >> 1;

        match result == 0 {
            true => self.set_zero(),
            false => self.unset_zero()
        };

        self.unset_subtraction();
        self.unset_half_carry();

        match v & 0x01 == 0 {
            true => self.unset_full_carry(),
            false => self.set_full_carry()
        };

        result
    }

    /// BIT - Store the complement of bit b of s in the Zero (Z) flag
    pub fn bit(&mut self, s: u8, b: u8) {
        match (s & (1 << b)) == 0 {
            true => self.set_zero(),
            false => self.unset_zero()
        };
        self.unset_subtraction();
        self.set_half_carry();
    }

    /*  Program Counter (PC) */

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

    /*   Stack Pointer (SP)  */

    /// Push 16 bits to the stack (SP)
    pub fn push_sp(&mut self, v: u16) {
        let value = Z80::u8_pair(v);
        self.registers.sp -= 2;
        self.mmu.write(value.1, self.registers.sp);
        self.mmu.write(value.0, self.registers.sp + 1);
    }

    /// Pop and return 16 bits from the stack (SP)
    pub fn pop_sp(&mut self) -> u16 {
        let lower = self.mmu.read(self.registers.sp);
        let upper = self.mmu.read(self.registers.sp + 1);
        self.registers.sp += 2;
        Z80::u16_from_u8(upper, lower)
    }

    /*        Control       */

    // RST - Restore the PC by popping the stack by 16 bits to acquire a previously-pushed location
    pub fn rst(&mut self, rst: u16) {
        self.push_sp(self.registers.pc);
        self.registers.pc = rst;
    }

    /// CALL - Store the current PC address on the stack and move PC to the given address
    pub fn call(&mut self, addr: u16) {
        self.push_sp(self.registers.pc);
        self.registers.pc = addr;
    }

    /// RET - Pop the stack by 16 bits and set the PC to the result
    pub fn ret(&mut self) {
        self.registers.pc = self.pop_sp();
    }

    /// JR - Adjust the PC by the given i8
    pub fn jr(&mut self, s: i8) {
        self.registers.pc = ((self.registers.pc as u32 as i32) + (s as i32)) as u16;
    }

    /*************************/
    /*      Conversions      */
    /*************************/

    /// Convert two u8s (given in the order of higher order, lower order) into a u16
    pub fn u16_from_u8(x: u8, y: u8) -> u16 {
        ((x as u16) << 8) + (y as u16)
    }

    /// Convert a u16 into two u8s (given in the order of higher order, lower order)
    pub fn u8_pair(x: u16) -> (u8, u8) {
        ((x >> 8) as u8, x as u8)
    }

    /*************************/
    /*     Register Pairs    */
    /*************************/

    /// Get the register pair AF as a u16
    pub fn get_af(&self) -> u16 {
        Z80::u16_from_u8(self.registers.a, self.registers.f)
    }

    /// Set the register pair AF to the given u16
    pub fn set_af(&mut self, x: u16) {
        let u8_pair = Z80::u8_pair(x);
        self.registers.a = u8_pair.0;
        self.registers.f = u8_pair.1;
    }

    /// Get the register pair BC as a u16
    pub fn get_bc(&self) -> u16 {
        Z80::u16_from_u8(self.registers.b, self.registers.c)
    }

    /// Set the register pair BC to the given u16
    pub fn set_bc(&mut self, x: u16) {
        let u8_pair = Z80::u8_pair(x);
        self.registers.b = u8_pair.0;
        self.registers.c = u8_pair.1;
    }

    /// Get the register pair DE as a u16
    pub fn get_de(&self) -> u16 {
        Z80::u16_from_u8(self.registers.d, self.registers.e)
    }

    /// Set the register pair DE to the given u16
    pub fn set_de(&mut self, x: u16) {
        let u8_pair = Z80::u8_pair(x);
        self.registers.d = u8_pair.0;
        self.registers.e = u8_pair.1;
    }

    /// Get the register pair HL as a u16
    pub fn get_hl(&self) -> u16 {
        Z80::u16_from_u8(self.registers.h, self.registers.l)
    }

    /// Set the register pair HL to the given u16
    pub fn set_hl(&mut self, x: u16) {
        let u8_pair = Z80::u8_pair(x);
        self.registers.h = u8_pair.0;
        self.registers.l = u8_pair.1;
    }

    /*************************/
    /*   Z/N/H/C Flags (F)   */
    /*************************/

    /*     Zero (Z) Flag     */

    /// Set the Zero (Z) flag of the F register
    pub fn set_zero(&mut self) {
        self.registers.f |= 0x80;
    }

    /// Unset the Zero (Z) flag of the F register
    pub fn unset_zero(&mut self) {
        self.registers.f &= !0x80;
    }

    /// Check the Zero(Z) flag of the F register
    pub fn is_zero(&self) -> bool {
        self.registers.f & 0x80 != 0
    }

    /*   Subtract (N) flag   */

    /// Set the Subtract (N) flag of the F register
    pub fn set_subtraction(&mut self) {
        self.registers.f |= 0x40;
    }

    /// Unset the Subtract (N) flag of the F register
    pub fn unset_subtraction(&mut self) {
        self.registers.f &= !0x40;
    }

    /// Check the Subtract (N) flag of the F register
    pub fn is_subtraction(&self) -> bool {
        self.registers.f & 0x40 != 0
    }

    /*  Half Carry (H)  Flag */

    /// Set the Half Carry (H) flag of the F register
    pub fn set_half_carry(&mut self) {
        self.registers.f |= 0x20;
    }

    /// Unset the Half Carry (H) flag of the F register
    pub fn unset_half_carry(&mut self) {
        self.registers.f &= !0x20;
    }

    /// Check the Half Carry (H) flag of the F register
    pub fn is_half_carry(&self) -> bool {
        self.registers.f & 0x20 != 0
    }

    /*     Carry (C) Flag    */

    /// Set the Carry (C) flag of the F register
    pub fn set_full_carry(&mut self) {
        self.registers.f |= 0x10;
    }

    /// Unset the Carry (C) flag of the F register
    pub fn unset_full_carry(&mut self) {
        self.registers.f &= !0x10;
    }

    /// Check the Carry (C) flag of the F register
    pub fn is_full_carry(&self) -> bool {
        self.registers.f & 0x10 != 0
    }

    /// Get the Interrupt Master Enable flag
    pub fn is_ime(&self) -> bool {self.ime}

    /// Set the Interrupt Master Enable flag
    pub fn set_ime(&mut self) {self.ime = true;}

    /// Unset the IME flag
    pub fn unset_ime(&mut self) {self.ime = false;}
}
