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
    pub clock: Clock,

    // Enum representing the Z80's current running status
    pub status: Status,

    // Struct representing the memory unit
    pub mmu: mmu::MMU
}


// Struct representing one instruction
struct Opcode {
    size: u16,                      // size in bytes of the opcode; should be 1, 2, 3, no larger
    clock_timing: Clock,            // the timing of m and t cycles taken in one instruction
    instruction: fn(&mut Z80)       // the actual function that will
}

impl Opcode {

    pub fn lookup(code: u8) -> Opcode {
        match code {
            0x00 => Opcode::nop(),
            0x01 => Opcode::ld_bc(),
            0x02 => Opcode::ld_bc_a(),
            _ => panic!("Unmapped opcode {}", code)
        }
    }

    // 0x00 - NOP
    fn nop() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: Clock {
                m: 1,
                t: 4
            },
            instruction: |_cpu: &mut Z80| { }
        }
    }

    // 0x01 - LD BC, d16
    fn ld_bc() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: Clock {
                m: 3,
                t: 12
            },
            instruction: |cpu: &mut Z80| {
                cpu.registers.c = cpu.mmu.read(cpu.registers.pc + 1);
                cpu.registers.b = cpu.mmu.read(cpu.registers.pc + 2);
            }
        }
    }

    // 0x02 - LD (BC), A
    fn ld_bc_a() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: Clock {
                m: 3,
                t: 12
            },
            instruction: |cpu: &mut Z80| {
                cpu.mmu.write(cpu.registers.a, ((cpu.registers.b << 8) + cpu.registers.c).into());
            }
        }
    }

    // 0x03 - INC BC
    fn inc_bc() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: Clock {
                m: 2,
                t: 8
            },
            instruction: |cpu: &mut Z80| {
                cpu.registers.c += 1;
                if cpu.registers.c == 0 {
                    cpu.registers.b += 1;
                }
            }
        }
    }


    // 0x20 - JR NZ s8
    fn jr_nz_s8() -> Opcode {
        Opcode {
            size: 0,    // Real: 2 bytes, but directly modified in instruction
            clock_timing: Clock {
                m: 3,
                t: 2
            },
            instruction: |cpu: &mut Z80| {
                match cpu.registers.f >> 7 {
                    0 => cpu.registers.pc += 2,
                    1 => cpu.registers.pc += cpu.mmu.read(cpu.registers.pc + 1) as i8,
                    _ => panic!("Somehow, the single-bit Z flag was neither 0 nor 1.")
                }
            }
        }
    }

    // 0x21 - LD SP d16
    fn ld_sp_d16() -> Opcode {
        Opcode {
            size: 3,
            clock_timing: Clock {
                m: 3,
                t: 0
            },
            instruction: |cpu: &mut Z80| {
                cpu.registers.h = cpu.mmu.read(cpu.registers.pc + 2);
                cpu.registers.l = cpu.mmu.read(cpu.registers.pc + 1);
            }
        }
    }

    // 0x22 - LD (HL+) A
    fn ld_hlp_a() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: Clock {
                m: 2,
                t: 8
            },
            instruction: |cpu: &mut Z80| {
                cpu.mmu.write(cpu.registers.a, ((cpu.registers.h << 8) + cpu.registers.l).into());
                match cpu.registers.l.checked_add(1) {
                    Some(x) => cpu.registers.l = x,
                    None => {
                        cpu.registers.l += 1;
                        cpu.registers.f |= 0x10;
                        match cpu.registers.h.checked_add(1) {
                            Some(x) => cpu.registers.h = x,
                            None => {
                                cpu.registers.h += 1;
                                cpu.registers.f |= 0x10;
                            }
                        }
                    }
                }
            }
        }
    }

    // 0x23 - INC HL
    fn inc_hl() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: Clock {
                m: 2,
                t: 8
            },
            instruction: |cpu: &mut Z80| {
                match cpu.registers.l.checked_add(1) {
                    Some(x) => cpu.registers.l = x,
                    None => {
                        cpu.registers.l += 1;
                        cpu.registers.f |= 0x10;
                        match cpu.registers.h.checked_add(1) {
                            Some(x) => cpu.registers.h = x,
                            None => {
                                cpu.registers.h += 1;
                                cpu.registers.f |= 0x10;
                            }
                        }
                    }
                }
            }
        }
    }

    // 0x24 - INC H
    fn inc_h() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: Clock {
                m: 1,
                t: 4
            },
            instruction: |cpu: &mut Z80| {
                match cpu.registers.h.checked_add(1) {
                    Some(x) => cpu.registers.h = x,
                    None => {
                        cpu.registers.h += 1;
                        cpu.registers.f |= 0x10;
                    }
                }
            }
        }
    }

}


#[allow(dead_code)]
impl Z80 {

    // Initialization / creation of a Z80 CPU
    pub fn init(mmu: mmu::MMU) -> Z80 {

        Z80 {

            // All registers begin empty
            registers: Registers {
                a: 0,
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                h: 0,
                l: 0,
                f: 0,
                pc: 0,
                sp: 0,
            },

            // Clock begins at 0
            clock: Clock { m: 0, t: 0 },

            // status enum starts as running.
            status: Status::RUNNING,

            // MMU Unit
            mmu
        }
    }

    // Basic execution of the current operation at the program-counter (PC) register
    pub fn step(&mut self) {

        // Get the opcode number to execute
        let opcode = self.mmu.read(self.registers.pc);

        // Fetch the opcode
        let opcode = instructions::Opcode::lookup(opcode);

        // Execute
        (opcode.instruction)(self);

        // Adjust clock and program counter (PC)
        self.clock.m += opcode.clock_timing.m;
        self.clock.t += opcode.clock_timing.t;

        self.registers.pc += opcode.size;
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

    pub fn zero(&mut self, result: u16) {
        match result {
            0 => self.set_zero(),
            _ => self.unset_zero()
        };
    }

    pub fn half_carry(&mut self, before: u8, after: u8) -> bool {
        (after >> 4) > (before >> 4)
    }

    /*************************/
    /*          ALU          */
    /*************************/

    // Add

    pub fn add_8(&mut self, s: u8, t: u8, flag: bool) -> u8 {

        let result = s.wrapping_add(t);

        if flag {
            self.zero(result as u16);

            /*
            self.o = false;
            self.h = false; // TODO - Detect half carry  (s & 0xF) + (t & 0xF) > 0xF,
            self.c = false; // TODO - detect carry (s as u16 + t as u16) > 0xFF
             */
        }

        result
    }

    pub fn add_16(&mut self, s: u16, t: u16, flag: bool) -> u16 {

        let result = s.wrapping_add(t);

        if flag {

            /*
            self.o = false;
            self.h = false; // TODO - Detect half carry  (s & 0xF) + (t & 0xF) > 0xF,
            self.c = false; // TODO - detect carry (s as u16 + t as u16) > 0xFF
             */
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

        }

        result
    }

    pub fn inc_16(&mut self, s: u16, flag: bool) -> u16 {

        let result = s.wrapping_add(1);

        if flag {

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





    // Register-specific arithmetic, setting necessary flags
    /*
        pub fn register_add(&mut self, source: u8, amount: u8) -> (u8, FlagResult, bool) {

            let mut result: u8;
            let mut carry: bool;

            let overflow = match source.checked_add(amount) {

                // No overflow - set as normal and return None
                Some(x) => {
                    carry = false;
                    result = x;
                    false
                },

                // Overflow, recompute, set flags
                None => {
                    carry = true;
                    result = source + amount;
                    true
                }
            };

            let flags = FlagResult {
                z: result == 0,
                o: false,
                c: carry,
                h: self.half_carry(source, result)
            };

            (result, flags, overflow)
        }

        pub fn register_sub(&mut self, source: u8, amount: u8) -> (u8, FlagResult, bool) {

            let mut result: u8;
            let mut carry: bool;

            let underflow = match register.checked_sub(amount) {

                // No underflow
                Some(x) => {
                    carry = false;
                    result = x;
                    false
                },

                // Underflow
                None => {
                    carry = true;
                    result = source - amount;
                    true
                }
            };

            let flags = FlagResult {
                z: result == 0,
                o: true,
                c: carry,
                h: self.half_carry(source, result)
            };

            (result, flags, underflow)
        }

        pub fn pair_add(&mut self, r_upper: u8, r_lower: u8, add_upper: u8, add_lower :u8) -> (u8, u8, FlagResult, bool) {

            let mut u_result = self.register_add(r_upper, add_upper);
            let mut l_result = self.register_add(r_lower, add_lower);

            // No overflow from lower into upper - finish early
            if let false = l_result.2 {

                u_result.1.z &= l_result.1.z;
                u_result.c |= l_result.1.c;
                u_result.h |= l_result.1.h;

                return (u_result.0, l_result.0, u_result.1, false);
            }

            // overflow from lower into upper
            let c_result = self.register_add(upper_result, l_result.0);

            u_result.1.c |= c_result.1.c;
            u_result.1.h |= c_result.1.h;

            match c_result.2 {

                true => {
                    u_result.1.z = true;
                    (0, 0, u_result.1, true)
                },

                false => {
                    u_result.1.z = false;
                    (c_result.0, 0, u_result.1, false)
                }
            }
        }

        pub fn pair_sub(&mut self, r_upper: &mut u8, r_lower: &mut u8, sub_upper: u8, sub_lower: u8) -> (u8, u8, FlagResult) {

            let original_half_carry = self.is_half_carry();

            if let Some(x) = self.register_sub(r_lower, sub_lower) {
                self.register_sub(r_upper, x);
            }

            self.register_sub(r_upper, sub_upper);

            match original_half_carry {
                true => self.set_half_carry(),
                false => self.unset_half_carry()
            };
        }
    */


    /*
    pub fn sub_16(s: u16, t: u16) -> (u16, FlagResult) {

        let result = s.wrapping_sub(t);

        (result, FlagResult {
            z: result == 0,
            o: true,
            h: (s &)
        })
    }*/


}
