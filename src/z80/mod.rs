use super::mmu;

#[derive(Debug)]
struct Clock {
    m: u16,
    t: u16
}


#[allow(dead_code)]
#[derive(Debug)]
enum Status{
    STOPPED,
    HALTED,
    RUNNING
}


#[derive(Debug)]
struct Registers{
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: u8,
    pc: u16,
    sp: u16,
}

// Struct representing the Z80 CPU
#[derive(Debug)]
struct Z80 {

    // Struct of all registers in the Z80
    registers: Registers,

    // Struct representing the clock of the Z80 for purposes of timing
    clock: Clock,

    // Enum representing the Z80's current running status
    status: Status,

    // Struct representing the memory unit
    mmu: mmu::MMU
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

            // 0x0X
            0x00 => Opcode::nop(),
            0x01 => Opcode::ld_bc(),
            0x02 => Opcode::ld_bc_a(),
            0x03 => Opcode::inc_bc(),
            0x04 => Opcode::inc_b(),
            0x05 => Opcode::dec_b(),
            0x06 => Opcode::ld_b(),
            0x07 => Opcode::rlca(),
            0x08 => Opcode::ld_a16_sp(),
            0x09 => Opcode::add_hl_bc(),
            0x0A => Opcode::ld_a_bc(),
            0x0B => Opcode::dec_bc(),
            0x0C => Opcode::inc_c(),
            0x0D => Opcode::dec_c(),
            0x0E => Opcode::ld_c(),
            0x0F => Opcode::rrca(),

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
            size: 3,
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

                let bc = cpu.get_bc();
                let bc = cpu.add_16(bc, 1, false);

                cpu.set_bc(bc);
            }
        }
    }

    // 0x04 - INC B
    fn inc_b() -> Opcode {
        Opcode{
            size: 1,
            clock_timing: Clock {
                m: 1,
                t: 4
            },
            instruction: |cpu: &mut Z80| {
                cpu.registers.b = cpu.add_8(cpu.registers.b, 1, true);
            }
        }
    }

    // 0x05 - DEC b
    fn dec_b() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: Clock {
                m: 1,
                t: 4
            },
            instruction: |cpu: &mut Z80| {
                cpu.registers.b = cpu.sub_8(cpu.registers.b, 1, true);
            }
        }
    }

    // 0x06 - LD B, d8
    fn ld_b() -> Opcode {
        Opcode {
            size: 2,
            clock_timing: Clock {
                m: 4,
                t: 8
            },
            instruction: |cpu: &mut Z80| {
                cpu.registers.b = cpu.mmu.read(cpu.registers.pc+1);
            }
        }
    }

    // 0x07 - RLCA
    fn rlca() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: Clock {
                m: 1,
                t: 4
            },
            instruction: |cpu: &mut Z80| {
                cpu.registers.a = (cpu.registers.a << 1) | (cpu.registers.a >> 7);
                cpu.unset_zero();
                cpu.unset_subtraction();
                cpu.unset_half_carry();

                match cpu.registers.a & 0x80 {
                    0x80 => cpu.set_full_carry(),
                    _ => cpu.unset_full_carry()
                };
            }
        }
    }

    // 0x08 - LD (a16), SP
    fn ld_a16_sp() -> Opcode {
        Opcode {
            size: 3,
            clock_timing: Clock {
                m: 5,
                t: 15
            },
            instruction: |cpu: &mut Z80| {
                let addr_lower = cpu.mmu.read(cpu.registers.pc + 1);
                let addr_upper = cpu.mmu.read(cpu.registers.pc + 2);
                let addr = ((addr_upper << 8) + addr_lower).into();

                cpu.mmu.write(cpu.registers.sp as u8, addr);
                cpu.mmu.write((cpu.registers.sp >> 8) as u8, addr);
            }
        }
    }

    // 0x09 - ADD HL, BC
    fn add_hl_bc() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: Clock {
                m: 2,
                t: 8
            },
            instruction: |cpu: &mut Z80| {

                let hl = cpu.get_hl();
                let bc = cpu.get_bc();

                let hl = cpu.sub_16(hl, bc, true);

                cpu.set_hl(hl);
            }
        }
    }

    // 0x0A - LD A, (BC)
    fn ld_a_bc() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: Clock {
                m: 2,
                t: 8
            },
            instruction: |cpu: &mut Z80| {
                cpu.registers.a = cpu.mmu.read(((cpu.registers.b << 8) + cpu.registers.c).into());
            }
        }
    }

    // 0x0B - DEC BC
    fn dec_bc() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: Clock {
                m: 2,
                t: 8
            },
            instruction: |cpu: &mut Z80| {

                let bc = cpu.get_bc();
                let bc = cpu.sub_16(bc, 1, false);

                cpu.set_bc(bc);
            }
        }
    }

    // 0x0C - INC C
    fn inc_c() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: Clock {
                m: 1,
                t: 4
            },
            instruction: |cpu: &mut Z80| {
                cpu.registers.c = cpu.inc_8(cpu.registers.c, true);
            }
        }
    }

    // 0x0D - DEC C
    fn dec_c() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: Clock {
                m: 1,
                t: 4
            },
            instruction: |cpu: &mut Z80| {
                cpu.registers.c = cpu.dec_8(cpu.registers.c, true);
            }
        }
    }

    // 0x0E - LD C, d8
    fn ld_c() -> Opcode {
        Opcode {
            size: 2,
            clock_timing: Clock {
                m: 2,
                t: 8
            },
            instruction: |cpu: &mut Z80| {
                cpu.registers.c = cpu.mmu.read(cpu.registers.pc+1);
            }
        }
    }

    // 0x0F - RRCA
    fn rrca() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: Clock {
                m: 1,
                t: 4
            },
            instruction: |cpu: &mut Z80| {
                cpu.registers.a = (cpu.registers.a >> 1) | (cpu.registers.a << 7);
                cpu.unset_zero();
                cpu.unset_subtraction();
                cpu.unset_half_carry();

                match cpu.registers.a >> 7 != 0{
                    true => cpu.set_full_carry(),
                    false => cpu.unset_full_carry()
                };
            }
        }
    }
}


#[allow(dead_code)]
impl Z80 {

    // Initialization / creation of a Z80 CPU
    fn init(mmu: mmu::MMU) -> Z80 {

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
    fn step(&mut self) {

        // Get the opcode number to execute
        let opcode = self.mmu.read(self.registers.pc);

        // Fetch the opcode
        let opcode = Opcode::lookup(opcode);

        // Execute
        (opcode.instruction)(self);

        // Adjust clock and program counter (PC)
        self.clock.m += opcode.clock_timing.m;
        self.clock.t += opcode.clock_timing.t;

        self.registers.pc += opcode.size;
    }

    // Set CPU F flags

    // Zero

    fn set_zero(&mut self) {
        self.registers.f |= 0x80;
    }

    fn unset_zero(&mut self) {
        self.registers.f ^= 0x80;
    }

    fn is_zero(&self) -> bool {
        self.registers.f & 0x80 != 0
    }

    // Subtraction

    fn set_subtraction(&mut self) {
        self.registers.f |= 0x40;
    }

    fn unset_subtraction(&mut self) {
        self.registers.f ^= 0x40;
    }

    fn is_subtraction(&self) -> bool {
        self.registers.f & 0x40 != 0
    }

    // Half Carry

    fn set_half_carry(&mut self) {
        self.registers.f |= 0x20;
    }

    fn unset_half_carry(&mut self) {
        self.registers.f ^= 0x20;
    }

    fn is_half_carry(&self) -> bool {
        self.registers.f & 0x20 != 0
    }

    // Full Carry

    fn set_full_carry(&mut self) {
        self.registers.f |= 0x10;
    }

    fn unset_full_carry(&mut self) {
        self.registers.f ^= 0x10;
    }

    fn is_full_carry(&self) -> bool {
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

    // Add

    fn add_8(&mut self, s: u8, t: u8, flag: bool) -> u8 {

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

    fn add_16(&mut self, s: u16, t: u16, flag: bool) -> u16 {

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

    fn sub_8(&mut self, s: u8, t: u8, flag: bool) -> u8 {

        let result = s.wrapping_sub(t);

        if flag {
            self.zero(result as u16);
        }

        result
    }

    fn sub_16(&mut self, s: u16, t: u16, flag: bool) -> u16 {

        let result = s.wrapping_sub(t);

        if flag {
            self.zero(result as u16);
        }

        result
    }

    // Inc

    fn inc_8(&mut self, s: u8, flag: bool) -> u8 {

        let result = s.wrapping_add(1);

        if flag {

        }

        result
    }

    fn inc_16(&mut self, s: u16, flag: bool) -> u16 {

        let result = s.wrapping_add(1);

        if flag {

        }

        result
    }

    // Dec

    fn dec_8(&mut self, s: u8, flag: bool) -> u8 {

        let result = s.wrapping_sub(1);

        if flag {

        }

        result
    }


    // Conversions

    fn u16_from_u8(x: u8, y: u8) -> u16 {
        ((x << 8) + y).into()
    }

    fn u8_pair(x: u16) -> (u8, u8) {
        ((x >> 8) as u8, x as u8)
    }

    /*************************/
    /*     Register Pairs    */
    /*************************/

    // AF
    fn get_af(&self) -> u16 {
        Z80::u16_from_u8(self.registers.a, self.registers.f)
    }

    fn set_af(&mut self, x: u16) {
        let u8_pair = Z80::u8_pair(x);
        self.registers.a = u8_pair.0;
        self.registers.f = u8_pair.1;
    }

    // BC
    fn get_bc(&self) -> u16 {
        Z80::u16_from_u8(self.registers.b, self.registers.c)
    }

    fn set_bc(&mut self, x: u16) {
        let u8_pair = Z80::u8_pair(x);
        self.registers.b = u8_pair.0;
        self.registers.c = u8_pair.1;
    }

    // DE
    fn get_de(&self) -> u16 {
        Z80::u16_from_u8(self.registers.d, self.registers.e)
    }

    fn set_de(&mut self, x: u16) {
        let u8_pair = Z80::u8_pair(x);
        self.registers.d = u8_pair.0;
        self.registers.e = u8_pair.1;
    }

    // HL
    fn get_hl(&self) -> u16 {
        Z80::u16_from_u8(self.registers.h, self.registers.l)
    }

    fn set_hl(&mut self, x: u16) {
        let u8_pair = Z80::u8_pair(x);
        self.registers.h = u8_pair.0;
        self.registers.l = u8_pair.1;
    }





    // Register-specific arithmetic, setting necessary flags
    /*
        fn register_add(&mut self, source: u8, amount: u8) -> (u8, FlagResult, bool) {

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

        fn register_sub(&mut self, source: u8, amount: u8) -> (u8, FlagResult, bool) {

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

        fn pair_add(&mut self, r_upper: u8, r_lower: u8, add_upper: u8, add_lower :u8) -> (u8, u8, FlagResult, bool) {

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

        fn pair_sub(&mut self, r_upper: &mut u8, r_lower: &mut u8, sub_upper: u8, sub_lower: u8) -> (u8, u8, FlagResult) {

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
    fn sub_16(s: u16, t: u16) -> (u16, FlagResult) {

        let result = s.wrapping_sub(t);

        (result, FlagResult {
            z: result == 0,
            o: true,
            h: (s &)
        })
    }*/


}
