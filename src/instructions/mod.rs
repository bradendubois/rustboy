use super::z80::{Z80};
use super::z80::Status;

impl Z80 {

    /// Call the instruction corresponding the given opcode, and return the number of cycles taken
    pub fn call_instruction(&mut self, code: u8) -> u64 {

        match self.use_cb_table {

            // Default LR35902 Opcodes
            false => {

                match code {

                    // 0x0X
                    0x00 => self.nop_0x00(),
                    0x01 => self.ld_bc_0x01(),
                    0x02 => self.ld_bc_a_0x02(),
                    0x03 => self.inc_bc_0x03(),
                    0x04 => self.inc_b_0x04(),
                    0x05 => self.dec_b_0x05(),
                    0x06 => self.ld_b_0x06(),
                    0x07 => self.rlca_0x07(),
                    0x08 => self.ld_a16_sp_0x08(),
                    0x09 => self.add_hl_bc_0x09(),
                    0x0A => self.ld_a_bc_0x0a(),
                    0x0B => self.dec_bc_0x0b(),
                    0x0C => self.inc_c_0x0c(),
                    0x0D => self.dec_c_0x0d(),
                    0x0E => self.ld_c_0x0e(),
                    0x0F => self.rrca_0x0f(),

                    // 0x1x
                    0x10 => self.stop_0x10(),
                    0x11 => self.ld_de_0x11(),
                    0x12 => self.ld_de_a_0x12(),
                    0x13 => self.inc_de_0x13(),
                    0x14 => self.inc_d_0x14(),
                    0x15 => self.dec_d_0x15(),
                    0x16 => self.ld_d_0x16(),
                    0x17 => self.rla_0x17(),
                    0x18 => self.jr_s8_0x18(),
                    0x19 => self.add_hl_de_0x19(),
                    0x1A => self.ld_a_de_0x1a(),
                    0x1B => self.dec_de_0x1b(),
                    0x1C => self.inc_e_0x1c(),
                    0x1D => self.dec_e_0x1d(),
                    0x1E => self.ld_e_d8_0x1e(),
                    0x1F => self.rra_0x1f(),

                    // 0x2X
                    0x20 => self.jr_nz_s8_0x20(),
                    0x21 => self.ld_hl_d16_0x21(),
                    0x22 => self.ld_hlp_a_0x22(),
                    0x23 => self.inc_hl_0x23(),
                    0x24 => self.inc_h_0x24(),
                    0x25 => self.dec_h_0x25(),
                    0x26 => self.ld_h_d8_0x26(),
                    0x27 => self.daa_0x27(),
                    0x28 => self.jr_z_s8_0x28(),
                    0x29 => self.add_hl_hl_0x29(),
                    0x2A => self.ld_a_hlp_0x2a(),
                    0x2B => self.dec_hl_0x2b(),
                    0x2C => self.inc_l_0x2c(),
                    0x2D => self.dec_l_0x2d(),
                    0x2E => self.ld_l_d8_0x2e(),
                    0x2F => self.cpl_0x2f(),

                    //0x3x
                    0x30 => self.jr_nc_s8_0x30(),
                    0x31 => self.ld_sp_d16_0x31(),
                    0x32 => self.ld_hls_a_0x32(),
                    0x33 => self.inc_sp_0x33(),
                    0x34 => self.inc_hl_0x34(),
                    0x35 => self.dec_hl_0x35(),
                    0x36 => self.ld_hl_d8_0x36(),
                    0x37 => self.scf_0x37(),
                    0x38 => self.jr_c_s8_0x38(),
                    0x39 => self.add_hl_sp_0x39(),
                    0x3A => self.ld_a_hls_0x3a(),
                    0x3B => self.dec_sp_0x3b(),
                    0x3C => self.inc_a_0x3c(),
                    0x3D => self.dec_a_0x3d(),
                    0x3E => self.ld_a_d8_0x3e(),
                    0x3F => self.ccf_0x3f(),

                    // 0x4X
                    0x40 => self.ld_b_b_0x40(),
                    0x41 => self.ld_b_c_0x41(),
                    0x42 => self.ld_b_d_0x42(),
                    0x43 => self.ld_b_e_0x43(),
                    0x44 => self.ld_b_h_0x44(),
                    0x45 => self.ld_b_l_0x45(),
                    0x46 => self.ld_b_hl_0x46(),
                    0x47 => self.ld_b_a_0x47(),
                    0x48 => self.ld_c_b_0x48(),
                    0x49 => self.ld_c_c_0x49(),
                    0x4A => self.ld_c_d_0x4a(),
                    0x4B => self.ld_c_e_0x4b(),
                    0x4C => self.ld_c_h_0x4c(),
                    0x4D => self.ld_c_l_0x4d(),
                    0x4E => self.ld_c_hl_0x4e(),
                    0x4F => self.ld_c_a_0x4f(),

                    // 0x6X
                    0x60 => self.ld_h_b_0x60(),
                    0x61 => self.ld_h_c_0x61(),
                    0x62 => self.ld_h_d_0x62(),
                    0x63 => self.ld_h_e_0x63(),
                    0x64 => self.ld_h_h_0x64(),
                    0x65 => self.ld_h_l_0x65(),
                    0x66 => self.ld_h_hl_0x66(),
                    0x67 => self.ld_h_a_0x67(),
                    0x68 => self.ld_l_b_0x68(),
                    0x69 => self.ld_l_c_0x69(),
                    0x6A => self.ld_l_d_0x6a(),
                    0x6B => self.ld_l_e_0x6b(),
                    0x6C => self.ld_l_h_0x6c(),
                    0x6D => self.ld_l_l_0x6d(),
                    0x6E => self.ld_l_hl_0x6e(),
                    0x6F => self.ld_l_a_0x6f(),

                    // 0x8X
                    0x80 => self.add_a_b_0x80(),
                    0x81 => self.add_a_c_0x81(),
                    0x82 => self.add_a_d_0x82(),
                    0x83 => self.add_a_e_0x83(),
                    0x84 => self.add_a_h_0x84(),
                    0x85 => self.add_a_l_0x85(),
                    0x86 => self.add_a_hl_0x86(),
                    0x87 => self.add_a_a_0x87(),
                    0x88 => self.adc_a_b_0x88(),
                    0x89 => self.adc_a_c_0x89(),
                    0x8A => self.adc_a_d_0x8a(),
                    0x8B => self.adc_a_e_0x8b(),
                    0x8C => self.adc_a_h_0x8c(),
                    0x8D => self.adc_a_l_0x8d(),
                    0x8E => self.adc_a_hl_0x8e(),
                    0x8F => self.adc_a_a_0x8f(),

                    // 0xAX
                    0xA0 => self.and_b_0xa0(),
                    0xA1 => self.and_c_0xa1(),
                    0xA2 => self.and_d_0xa2(),
                    0xA3 => self.and_e_0xa3(),
                    0xA4 => self.and_h_0xa4(),
                    0xA5 => self.and_l_0xa5(),
                    0xA6 => self.and_hl_0xa6(),
                    0xA7 => self.and_a_0xa7(),
                    0xA8 => self.xor_b_0xa8(),
                    0xA9 => self.xor_c_0xa9(),
                    0xAA => self.xor_d_0xaa(),
                    0xAB => self.xor_e_0xab(),
                    0xAC => self.xor_h_0xac(),
                    0xAD => self.xor_l_0xad(),
                    0xAE => self.xor_hl_0xae(),
                    0xAF => self.xor_a_0xaf(),

                    // 0xCX
                    0xC0 => self.ret_nz_0xc0(),
                    0xC1 => self.pop_bc_0xc1(),
                    0xC2 => self.jp_nz_a16_0xc2(),
                    0xC3 => self.jp_a16_0xc3(),
                    0xC4 => self.call_nz_a16_0xc4(),
                    0xC5 => self.push_bc_0xc5(),
                    0xC6 => self.add_a_d8_0xc6(),
                    0xC7 => self.rst_00h_0xc7(),
                    0xC8 => self.ret_z_0xc8(),
                    0xC9 => self.ret_0xc9(),
                    0xCA => self.jp_z_a16_0xca(),
                    0xCB => self.cb(),
                    0xCC => self.call_z_a16_0xcc(),
                    0xCD => self.call_a16_0xcd(),
                    0xCE => self.call_adc_a_d8_0xce(),
                    0xCF => self.rst_08h_0xcf(),

                    // 0xEX
                    0xE0 => self.ldh_a8_a_0xe0(),
                    0xE1 => self.pop_hl_0xe1(),
                    0xE2 => self.ld_c_a_0xe2(),
                    0xE5 => self.push_hl_0xe5(),
                    0xE6 => self.and_d8_0xe6(),
                    0xE7 => self.rst_20h_0xe7(),
                    0xE8 => self.add_sp_r8_0xe8(),
                    0xE9 => self.jp_hl_0xe9(),
                    0xEA => self.ld_a16_a_0xea(),
                    0xEE => self.xor_d8_0xee(),
                    0xEF => self.rst_28h_0xef(),


                    // Unmapped code in default table
                    _ => panic!("Unmapped default table opcode {}", code)
                }
            },

            // CB Prefix Table
            true => {

                // Can safely unset the flag and interpret the *next* instruction normally
                self.use_cb_table = false;

                match code {

                    // Dummy line just to test
                    0x00 => 3,

                    _ => panic!("Unmapped CB prefix opcode {}", code)
                }
            }
        }
    }

    // 0x00 - NOP
    pub fn nop_0x00(&mut self) -> u64 {
        4
    }

    // 0x01 - LD BC, d16
    fn ld_bc_0x01(&mut self) -> u64 {
        self.registers.c = self.byte();
        self.registers.b = self.byte();
        12
    }

    // 0x02 - LD (BC), A
    fn ld_bc_a_0x02(&mut self) -> u64 {
        self.mmu.write(self.registers.a, self.get_bc());
        8
    }

    // 0x03 - INC BC
    fn inc_bc_0x03(&mut self) -> u64 {
        let bc = self.inc_16(self.get_bc());
        self.set_bc(bc);
        8
    }

    // 0x04 - INC B
    fn inc_b_0x04(&mut self) -> u64 {
        self.registers.b = self.inc_8(self.registers.b);
        4
    }

    // 0x05 - DEC b
    fn dec_b_0x05(&mut self) -> u64 {
        self.registers.b = self.dec_8(self.registers.b);
        4
    }

    // 0x06 - LD B, d8
    fn ld_b_0x06(&mut self) -> u64 {
        self.registers.b = self.byte();
        8
    }

    // 0x07 - RLCA
    fn rlca_0x07(&mut self) -> u64 {
        self.registers.a = (self.registers.a << 1) | (self.registers.a >> 7);
        self.unset_zero();
        self.unset_subtraction();
        self.unset_half_carry();

        match self.registers.a & 0x80 {
            0x80 => self.set_full_carry(),
            _ => self.unset_full_carry()
        };
        4
    }

    // 0x08 - LD (a16), SP
    fn ld_a16_sp_0x08(&mut self) -> u64 {
        let addr = self.word();
        self.mmu.write(self.registers.sp as u8, addr);
        self.mmu.write((self.registers.sp >> 8) as u8, addr + 1);
        20
    }

    // 0x09 - ADD HL, BC
    fn add_hl_bc_0x09(&mut self) -> u64 {
        let hl = self.get_hl();
        let bc = self.get_bc();
        let hl = self.add_16(hl, bc);
        self.set_hl(hl);
        8
    }

    // 0x0A - LD A, (BC)
    fn ld_a_bc_0x0a(&mut self) -> u64 {
        self.registers.a = self.mmu.read(self.get_bc());
        8
    }

    // 0x0B - DEC BC
    fn dec_bc_0x0b(&mut self) -> u64 {
        let bc = self.dec_16(self.get_bc());
        self.set_bc(bc);
        8
    }

    // 0x0C - INC C
    fn inc_c_0x0c(&mut self) -> u64 {
        self.registers.c = self.inc_8(self.registers.c);
        4
    }

    // 0x0D - DEC C
    fn dec_c_0x0d(&mut self) -> u64 {
        self.registers.c = self.dec_8(self.registers.c);
        4
    }

    // 0x0E - LD C, d8
    fn ld_c_0x0e(&mut self) -> u64 {
        self.registers.c = self.byte();
        8
    }

    // 0x0F - RRCA
    fn rrca_0x0f(&mut self) -> u64 {
        self.registers.a = (self.registers.a >> 1) | (self.registers.a << 7);
        self.unset_zero();
        self.unset_subtraction();
        self.unset_half_carry();
        match self.registers.a >> 7 != 0{
            true => self.set_full_carry(),
            false => self.unset_full_carry()
        };
        4
    }

    /// 0x10 - STOP : Stops the system clock and oscillator circuit.
    /// LCD controller is also stopped.
    /// Internal RAM register ports remain unchanged
    /// Cancelled by RESET signal
    fn stop_0x10(&mut self) -> u64 {
            self.status = Status::STOPPED;
            4
    }

    /// 0x11 - LD DE, d16 : Loads 2 bytes of immediate data into registers D,E
    /// First byte is the lower byte, second byte is higher. Love Little endian -.-
    fn ld_de_0x11(&mut self) -> u64 {
        self.registers.d = self.byte();
        self.registers.e = self.byte();
        12
    }

    /// 0x12 - LD (DE), A : store contents of A in memory location specified by registers DE
    fn ld_de_a_0x12(&mut self) -> u64 {
        self.mmu.write(self.registers.a, self.get_de());
        8
    }

    /// 0x13 - INC DE : Increment the contents of registers DE by 1
    fn inc_de_0x13(&mut self) -> u64 {
        let de = self.inc_16(self.get_de());
        self.set_de(de);
        8
    }

    /// 0x14 - INC D : Increment the contents of D
    fn inc_d_0x14(&mut self) -> u64 {
        self.registers.d = self.inc_8(self.registers.d);
        4
    }

    /// 0x15 - DEC D: Decrement the register D
    fn dec_d_0x15(&mut self) -> u64 {
        self.registers.d = self.dec_8(self.registers.d);
        4
    }

    /// 0x16 - LD D, d8: Load the 8-bit immediate operand d8 into reg D
    fn ld_d_0x16(&mut self) -> u64 {
        self.registers.d = self.byte();
        8
    }

    ///0x17 - RLA : Rotate contents of register A to the left,
    fn rla_0x17(&mut self) -> u64 {
        self.unset_zero();
        self.unset_subtraction();
        self.unset_half_carry();
        let temp = self.is_full_carry();
        if self.registers.a & 0x80 == 1 {
            self.set_full_carry()
        } else {
            self.unset_full_carry()
        }
        self.registers.a = self.registers.a << 1;
        self.registers.a |= temp as u8;
        4
    }

    ///0x18 - JR s8 : Jump s8 steps from current address in program counter
    fn jr_s8_0x18(&mut self) -> u64 {

        let next = self.byte() as i8;
        self.jr(next);
        12
    }

    ///0x19 - ADD HL DE : add the contents of de to hl
    fn add_hl_de_0x19(&mut self) -> u64 {
        let val = self.add_16(self.get_hl(), self.get_de());
        self.set_hl(val);
        8
    }

    ///0x1A - LD A, (DE) : Load the 8-bit contents of memory specified by de into a
    fn ld_a_de_0x1a(&mut self) -> u64 {
        self.registers.a = self.mmu.read(self.get_de());
        8
    }

    /// 0x1B - DEC DE : decrement contents of de by 1!
    ///
    fn dec_de_0x1b(&mut self) -> u64 {
        let de = self.get_de();
        self.set_de(de);
        8

    }

    /// 0x1C - INC E
    fn inc_e_0x1c(&mut self) -> u64 {
        self.registers.e = self.inc_8(self.registers.e);
        4
    }

    ///0x1D - DEC E
    fn dec_e_0x1d(&mut self) -> u64 {
        self.registers.e = self.dec_8(self.registers.e);
        4
    }

    ///0x1E - LD E d8 : load 8 bit operand d8 into e
    fn ld_e_d8_0x1e(&mut self) -> u64 {
        self.registers.e = self.byte();
        8

    }

    ///0x1F - RRA : rotate register A to the right,
    /// through the carry flag,
    fn rra_0x1f(&mut self) -> u64 {
        let temp = self.is_full_carry();
        self.unset_zero();
        self.unset_subtraction();
        self.unset_half_carry();
        if self.registers.a & 0x01 != 0 {self.set_full_carry()} else{self.unset_full_carry()}
        self.registers.a = self.registers.a | (temp as u8) << 7;
        4
    }
    // 0x20 - JR NZ s8
    fn jr_nz_s8_0x20(&mut self) -> u64 {

        let next = self.byte() as i8;

        match self.is_zero() {
            true => {
                self.jr(next);
                12
            },
            false => 8
        }
    }

    // 0x21 - LD HL d16
    fn ld_hl_d16_0x21(&mut self) -> u64 {
        self.registers.h = self.byte();
        self.registers.l = self.byte();
        12
    }

    // 0x22 - LD (HL+) A
    fn ld_hlp_a_0x22(&mut self) -> u64 {
        let hl = self.get_hl();
        self.mmu.write(self.registers.a, hl);
        let hl = self.inc_16(hl);
        self.set_hl(hl);
        8
    }

    // 0x23 - INC HL
    fn inc_hl_0x23(&mut self) -> u64 {
        let hl = self.inc_16(self.get_hl());
        self.set_hl(hl);
        8
    }

    // 0x24 - INC H
    fn inc_h_0x24(&mut self) -> u64 {
        self.registers.h = self.inc_8(self.registers.h);
        4
    }

    // 0x25 - DEC H
    fn dec_h_0x25(&mut self) -> u64 {
        self.registers.h = self.dec_8(self.registers.h);
        4
    }

    // 0x26 - LD H d8
    fn ld_h_d8_0x26(&mut self) -> u64 {
        self.registers.h = self.byte();
        8
    }

    // 0x27 - DAA
    fn daa_0x27(&mut self) -> u64 {

        let mut adj = 0x00;

        if self.is_full_carry() { adj |= 0x60; }
        if self.is_half_carry() { adj |= 0x06; }

        if !self.is_subtraction() {
            if self.registers.a & 0x0F > 0x09 { adj |= 0x06; };
            if self.registers.a > 0x99 { adj |= 0x60; };
        }

        self.registers.a = self.registers.a.wrapping_add(adj);

        match adj >= 0x60 {
            true => self.set_full_carry(),
            false => self.unset_full_carry()
        };

        self.unset_half_carry();

        match self.registers.a == 0 {
            true => self.set_zero(),
            false => self.unset_zero()
        };

        4
    }

    // 0x28 - JR Z s8
    fn jr_z_s8_0x28(&mut self) -> u64 {

        let next = self.byte() as i8;

        match self.is_zero() {
            true => 8,
            false => {
                self.jr(next);
                12
            }
        }
    }

    // 0x29 - ADD HL HL
    fn add_hl_hl_0x29(&mut self) -> u64 {
        let hl = self.get_hl();
        let hl = self.add_16(hl, hl);
        self.set_hl(hl);
        8
    }

    // 0x2A LD A HL+
    fn ld_a_hlp_0x2a(&mut self) -> u64 {
        let hl = self.get_hl();
        self.registers.a = self.mmu.read(hl);
        let hl = self.inc_16(hl);
        self.set_hl(hl);
        8
    }

    // 0x2B - DEC HL
    fn dec_hl_0x2b(&mut self) -> u64 {
        let hl = self.dec_16(self.get_hl());
        self.set_hl(hl);
        8
    }

    // 0x2C - INC L
    fn inc_l_0x2c(&mut self) -> u64 {
        self.registers.l = self.inc_8(self.registers.l);
        4
    }

    // 0x2D - DEC L
    fn dec_l_0x2d(&mut self) -> u64 {
        self.registers.l = self.dec_8(self.registers.l);
        4
    }

    // 0x2E - LD L d8
    fn ld_l_d8_0x2e(&mut self) -> u64 {
        self.registers.l = self.byte();
        8
    }

    // 0x2F - CPL
    fn cpl_0x2f(&mut self) -> u64 {
        self.registers.a = !self.registers.a;
        4
    }

    // 0x30 - JR NC, s8 : Jump s8 if carry flag is 0
    fn jr_nc_s8_0x30(&mut self) -> u64 {
        let next = self.byte() as i8;

        match !self.is_full_carry() {
            true => {
                self.jr(next);
                12
            },
            false => 8
        }
    }
    // 0x31 - LD SP, d16 : Load the 2 bytes of immediate data into register pair SP
    fn ld_sp_d16_0x31(&mut self) -> u64 {
        self.registers.sp = self.word();
        12
    }

    // 0x32 - LD HL(-), A
    fn ld_hls_a_0x32(&mut self) -> u64 {
        self.mmu.write(self.registers.a, self.get_hl());
        let hl = self.dec_16(self.get_hl());
        self.set_hl(hl);
        8
    }

    // 0x33 - INC SP
    fn inc_sp_0x33(&mut self) -> u64 {
        self.registers.sp = self.inc_16(self.registers.sp);
        8
    }

    // 0x34 - INC (HL)
    fn inc_hl_0x34(&mut self) -> u64 {
        let mut hl = self.mmu.read(self.get_hl());
        hl = self.inc_8(hl);
        self.mmu.write(hl, self.get_hl());
        12
    }

    //0x35 - DEC (HL)
    fn dec_hl_0x35(&mut self) -> u64 {
        let mut hl = self.mmu.read(self.get_hl());
        hl = self.dec_8(hl);
        self.mmu.write(hl, self.get_hl());
        12
    }

    // 0x36 - LD HL, d8
    fn ld_hl_d8_0x36(&mut self) -> u64 {
        let d8 = self.byte();
        self.mmu.write(d8, self.get_hl());
        12
    }

    //0x37 - SCF
    fn scf_0x37(&mut self) -> u64 {
        self.set_full_carry();
        self.unset_half_carry();
        self.unset_subtraction();
        4
    }

    // 0x38 JR C, s8
    fn jr_c_s8_0x38(&mut self) -> u64 {
        match self.is_full_carry(){
            true => {
                let s8 = self.byte();
                self.jr(s8 as i8);
                12
            },
            false => 8
        }
    }

    // 0x39 - ADD HL SP
    fn add_hl_sp_0x39(&mut self) -> u64{
        let sp = self.registers.sp;
        let hl = self.add_16(hl, sp);
        self.set_hl(hl);
        8
    }

    //0x3A - LD A, (HL-)
    fn ld_a_hls_0x3a(&mut self) -> u64{
        let mut hl = self.get_hl();
        self.registers.a = self.mmu.read(hl);
        hl = self.inc_16(hl);
        self.set_hl(hl);
        8
    }

    // 0x3B - DEC SP
    fn dec_sp_0x3b(&mut self) -> u64{
        self.registers.sp = self.dec_16(self.registers.sp);
        8
    }

    //0x3C - INC A
    fn inc_a_0x3c(&mut self) -> u64{
        self.registers.a = self.inc_8(self.registers.a);
        4
    }

    //0x3D - DEC A
    fn dec_a_0x3d(&mut self) -> u64 {
        self.registers.a = self.dec_8(self.registers.a);
        4
    }

    //0x3E - LD A, d8
    fn ld_a_d8_0x3e(&mut self) -> u64 {
        self.registers.a = self.byte();
        8
    }

    // 0x3F - CCF
    fn ccf_0x3f(&mut self) -> u64 {
        match self.is_full_carry(){
            true => self.unset_full_carry(),
            false => self.set_full_carry()
        };
        self.unset_subtraction();
        self.unset_half_carry();
        4
    }



    // 0x40 - LD B B
    fn ld_b_b_0x40(&mut self) -> u64 {
        self.registers.b = self.registers.b;      // ah, yes
        4
    }

    // 0x41 - LD B C
    fn ld_b_c_0x41(&mut self) -> u64 {
        self.registers.b = self.registers.c;
        4
    }

    // 0x42 - LD B D
    fn ld_b_d_0x42(&mut self) -> u64 {
        self.registers.b = self.registers.d;
        4
    }

    // 0x43 - LD B E
    fn ld_b_e_0x43(&mut self) -> u64 {
        self.registers.b = self.registers.e;
        4
    }

    // 0x44 - LD B H
    fn ld_b_h_0x44(&mut self) -> u64 {
        self.registers.b = self.registers.h;
        4
    }

    // 0x45 - LD B L
    fn ld_b_l_0x45(&mut self) -> u64 {
        self.registers.b = self.registers.l;
        4
    }

    // 0x46 - LD B (HL)
    fn ld_b_hl_0x46(&mut self) -> u64 {
        self.registers.b = self.byte();
        8
    }

    // 0x47 - LD B A
    fn ld_b_a_0x47(&mut self) -> u64 {
        self.registers.b = self.registers.a;
        4
    }

    // 0x48 - LD C B
    fn ld_c_b_0x48(&mut self) -> u64 {
        self.registers.c = self.registers.b;
        4
    }

    // 0x49 - LD C C
    fn ld_c_c_0x49(&mut self) -> u64 {
        self.registers.c = self.registers.c;      // ok
        4
    }

    // 0x4A - LD C D
    fn ld_c_d_0x4a(&mut self) -> u64 {
        self.registers.c = self.registers.d;
        4
    }

    // 0x4B - LD C E
    fn ld_c_e_0x4b(&mut self) -> u64 {
        self.registers.c = self.registers.e;
        4
    }

    // 0x4C - LD C H
    fn ld_c_h_0x4c(&mut self) -> u64 {
        self.registers.c = self.registers.h;
        4
    }

    // 0x4D - LD C L
    fn ld_c_l_0x4d(&mut self) -> u64 {
        self.registers.c = self.registers.l;
        4
    }

    // 0x4E - LD C (HL)
    fn ld_c_hl_0x4e(&mut self) -> u64 {
        self.registers.c = self.mmu.read(self.get_hl());
        8
    }

    // 0x4F - LD C A
    fn ld_c_a_0x4f(&mut self) -> u64 {
        self.registers.c = self.registers.a;
        4
    }

    // 0x60 - LD H B
    fn ld_h_b_0x60(&mut self) -> u64 {
        self.registers.h = self.registers.b;
        4
    }

    // 0x61 - LD H C
    fn ld_h_c_0x61(&mut self) -> u64 {
        self.registers.h = self.registers.c;
        4
    }

    // 0x62 - LD H D
    fn ld_h_d_0x62(&mut self) -> u64 {
        self.registers.h = self.registers.d;
        4
    }

    // 0x63 - LD H E
    fn ld_h_e_0x63(&mut self) -> u64 {
        self.registers.h = self.registers.e;
        4
    }

    // 0x64 - LD H H
    fn ld_h_h_0x64(&mut self) -> u64 {
        self.registers.h = self.registers.h;      // sure
        4
    }

    // 0x65 - LD H L
    fn ld_h_l_0x65(&mut self) -> u64 {
        self.registers.h = self.registers.l;
        4
    }

    // 0x66 - LD H (HL)
    fn ld_h_hl_0x66(&mut self) -> u64 {
        self.registers.h = self.byte();
        8
    }

    // 0x67 - LD H A
    fn ld_h_a_0x67(&mut self) -> u64 {
        self.registers.h = self.registers.a;
        4
    }

    // 0x68 - LD L B
    fn ld_l_b_0x68(&mut self) -> u64 {
        self.registers.l = self.registers.b;
        4
    }

    // 0x69 - LD L C
    fn ld_l_c_0x69(&mut self) -> u64 {
        self.registers.l = self.registers.c;
        4
    }

    // 0x6A - LD L D
    fn ld_l_d_0x6a(&mut self) -> u64 {
        self.registers.l = self.registers.d;
        4
    }

    // 0x6B - LD L E
    fn ld_l_e_0x6b(&mut self) -> u64 {
        self.registers.l = self.registers.e;
        4
    }

    // 0x6C - LD L H
    fn ld_l_h_0x6c(&mut self) -> u64 {
        self.registers.l = self.registers.h;
        4
    }

    // 0x6D - LD L L
    fn ld_l_l_0x6d(&mut self) -> u64 {
        self.registers.l = self.registers.l;      // ok
        4
    }

    // 0x6E - LD (HL)
    fn ld_l_hl_0x6e(&mut self) -> u64 {
        self.registers.l = self.mmu.read(self.get_hl());
        8
    }

    // 0x6F - LD L A
    fn ld_l_a_0x6f(&mut self) -> u64 {
        self.registers.l = self.registers.a;
        4
    }

    // 0x80 - ADD A,B
    fn add_a_b_0x80(&mut self) -> u64 {
        self.registers.a = self.add_8(self.registers.a, self.registers.b);
        4
    }

    // 0x81 - ADD A,C
    fn add_a_c_0x81(&mut self) -> u64 {
        self.registers.a = self.add_8(self.registers.a, self.registers.c);
        4
    }

    // 0x82 - ADD A,D
    fn add_a_d_0x82(&mut self) -> u64 {
        self.registers.a = self.add_8(self.registers.a, self.registers.d);
        4
    }

    // 0x83 - ADD A,E
    fn add_a_e_0x83(&mut self) -> u64 {
        self.registers.a = self.add_8(self.registers.a, self.registers.e);
        4
    }

    // 0x84 - ADD A,H
    fn add_a_h_0x84(&mut self) -> u64 {
        self.registers.a = self.add_8(self.registers.a, self.registers.h);
        4
    }

    // 0x85 - ADD A,L
    fn add_a_l_0x85(&mut self) -> u64 {
        self.registers.a = self.add_8(self.registers.a, self.registers.l);
        4
    }

    // 0x86 - ADD A,(HL)
    fn add_a_hl_0x86(&mut self) -> u64 {
        let value = self.mmu.read(self.get_hl());
        self.registers.a = self.add_8(self.registers.a, value);
        8
    }

    // 0x87 - ADD A,A
    fn add_a_a_0x87(&mut self) -> u64 {
        self.registers.a = self.add_8(self.registers.a, self.registers.a);
        4
    }

    // 0x88 - ADC A,B
    fn adc_a_b_0x88(&mut self) -> u64 {
        self.adc_8(self.registers.b);
        4
    }

    // 0x89 - ADC A,C
    fn adc_a_c_0x89(&mut self) -> u64 {
        self.adc_8(self.registers.c);
        4
    }

    // 0x8A - ADC A,D
    fn adc_a_d_0x8a(&mut self) -> u64 {
        self.adc_8(self.registers.d);
        4
    }

    // 0x8B - ADC A,E
    fn adc_a_e_0x8b(&mut self) -> u64 {
        self.adc_8(self.registers.e);
        4
    }

    // 0x8C - ADC A,H
    fn adc_a_h_0x8c(&mut self) -> u64 {
        self.adc_8(self.registers.h);
        4
    }

    // 0x8D - ADC A,L
    fn adc_a_l_0x8d(&mut self) -> u64 {
        self.adc_8(self.registers.l);
        4
    }

    // 0x8E - ADC A,(HL)
    fn adc_a_hl_0x8e(&mut self) -> u64 {
        let value = self.mmu.read(self.get_hl());
        self.adc_8(value);
        8
    }

    // 0x8F - ADC A,A
    fn adc_a_a_0x8f(&mut self) -> u64 {
        self.adc_8(self.registers.a);
        4
    }

    // 0xA0 - AND B
    fn and_b_0xa0(&mut self) -> u64 {
        self.and(self.registers.b);
        4
    }

    // 0xA1 - AND C
    fn and_c_0xa1(&mut self) -> u64 {
        self.and(self.registers.c);
        4
    }

    // 0xA2 - AND D
    fn and_d_0xa2(&mut self) -> u64 {
        self.and(self.registers.d);
        4
    }

    // 0xA3 - AND E
    fn and_e_0xa3(&mut self) -> u64 {
        self.and(self.registers.e);
        4
    }

    // 0xA4 - AND H
    fn and_h_0xa4(&mut self) -> u64 {
        self.and(self.registers.h);
        4
    }

    // 0xA5 - AND L
    fn and_l_0xa5(&mut self) -> u64 {
        self.and(self.registers.l);
        4
    }

    // 0xA6 - AND (HL)
    fn and_hl_0xa6(&mut self) -> u64 {
        let value = self.mmu.read(self.get_hl());
        self.and(value);
        8
    }

    // 0xA7 - AND A
    fn and_a_0xa7(&mut self) -> u64 {
        self.and(self.registers.a);   // ok
        4
    }

    // 0xA8 - XOR B
    fn xor_b_0xa8(&mut self) -> u64 {
        self.xor(self.registers.b);
        4
    }

    // 0xA9 - XOR C
    fn xor_c_0xa9(&mut self) -> u64 {
        self.xor(self.registers.c);
        4
    }

    // 0xAA - XOR D
    fn xor_d_0xaa(&mut self) -> u64 {
        self.xor(self.registers.d);
        4
    }

    // 0xAB - XOR E
    fn xor_e_0xab(&mut self) -> u64 {
        self.xor(self.registers.e);
        4
    }

    // 0xAC - XOR H
    fn xor_h_0xac(&mut self) -> u64 {
        self.xor(self.registers.h);
        4
    }

    // 0xAD - XOR L
    fn xor_l_0xad(&mut self) -> u64 {
        self.xor(self.registers.l);
        4
    }

    // 0xAE - XOR (HL)
    fn xor_hl_0xae(&mut self) -> u64 {
        let value = self.mmu.read(self.get_hl());
        self.xor(value);
        8
    }

    // 0xAF - XOR A
    fn xor_a_0xaf(&mut self) -> u64 {
        self.xor(self.registers.a);   // why not
        4
    }

    // 0xC0 - RET NZ
    fn ret_nz_0xc0(&mut self) -> u64 {
        match self.is_zero() {
            true => 8,
            false => {
                self.ret();
                20
            }
        }
    }

    // 0xC1 - POP BC
    fn pop_bc_0xc1(&mut self) -> u64 {
        let value = self.pop_sp();
        self.set_bc(value);
        12
    }

    // 0xC2 - JP NZ a16
    fn jp_nz_a16_0xc2(&mut self) -> u64 {
        let value = self.word();
        match self.is_zero() {
            false => {
                self.registers.pc = value;
                16
            },
            true => 12
        }
    }

    // 0xC3 - JP a16
    fn jp_a16_0xc3(&mut self) -> u64 {
        let value = self.word();
        self.registers.pc = value;
        16
    }

    // 0xC4 - CALL NZ a16
    fn call_nz_a16_0xc4(&mut self) -> u64 {
        let value = self.word();
        match self.is_zero() {
            true => {
                self.call(value);
                24
            },
            false => 12
        }
    }

    // 0xC5 - PUSH BC
    fn push_bc_0xc5(&mut self) -> u64 {
        self.push_sp(self.get_bc());
        16
    }

    // 0xC6 - ADD A d8
    fn add_a_d8_0xc6(&mut self) -> u64 {
        let value = self.byte();
        self.registers.a = self.add_8(self.registers.a, value);
        8
    }

    // 0xC7 - RST 00H
    fn rst_00h_0xc7(&mut self) -> u64 {
        self.rst(0x00);
        16
    }

    // 0xC8 - RET Z
    fn ret_z_0xc8(&mut self) -> u64 {
        match self.is_zero() {
            true => {
                self.ret();
                20
            },
            false => {
                8
            }
        }
    }

    // 0xC9 - RET
    fn ret_0xc9(&mut self) -> u64 {
        self.ret();
        16
    }

    // 0xCA - JP Z a16
    fn jp_z_a16_0xca(&mut self) -> u64 {
        let a16 = self.word();
        match self.is_zero() {
            true => {
                self.registers.pc = a16;
                16
            },
            false => 12
        }
    }

    // 0xCB - CB PREFIX
    fn cb(&mut self) -> u64 {
        self.use_cb_table = true;
        4
    }

    // 0xCC - CALL Z a16
    fn call_z_a16_0xcc(&mut self) -> u64 {
        let a16 = self.word();
        match self.is_zero() {
            true => {
                self.call(a16);
                24
            },
            false => 12
        }
    }

    // 0xCD - CALL a16
    fn call_a16_0xcd(&mut self) -> u64 {
        let a16 = self.word();
        self.call(a16);
        24
    }

    // 0xCE - ADC A d8
    fn call_adc_a_d8_0xce(&mut self) -> u64 {
        let d8 = self.byte();
        self.adc_8(d8);
        8
    }

    // 0xCF - RST 08H
    fn rst_08h_0xcf(&mut self) -> u64 {
        self.rst(0x08);
        16
    }

    // 0xE0 - LDH (a8) A
    fn ldh_a8_a_0xe0(&mut self) -> u64 {
        let a8 = self.byte();
        self.mmu.write(self.registers.a, 0xFF00 | a8 as u16);
        12
    }

    // 0xE1 - POP HL
    fn pop_hl_0xe1(&mut self) -> u64 {
        let value = self.pop_sp();
        self.set_hl(value);
        12
    }

    // 0xE2 - LD (C) A
    fn ld_c_a_0xe2(&mut self) -> u64 {
        self.mmu.write(self.registers.a, 0xFF00 | self.registers.c as u16);
        8
    }

    // 0xE5 - PUSH HL
    fn push_hl_0xe5(&mut self) -> u64 {
        self.push_sp(self.get_hl());
        16
    }

    // 0xE6 - AND d8
    fn and_d8_0xe6(&mut self) -> u64 {
        let d8 = self.byte();
        self.and(d8);
        8
    }

    // 0xE7 - RST 20H
    fn rst_20h_0xe7(&mut self) -> u64 {
        self.rst(0x20);
        16
    }

    // 0xE8 - ADD SP, r8
    fn add_sp_r8_0xe8(&mut self) -> u64 {
        let r8 = self.byte() as i8 as i16 as u16;
        self.registers.sp = self.add_16(self.registers.sp, r8);
        self.unset_zero();
        16
    }

    // 0xE9 - JP (HL)
    fn jp_hl_0xe9(&mut self) -> u64 {
        self.registers.pc = self.get_hl();
        4
    }

    // 0xEA - LD (a16) A
    fn ld_a16_a_0xea(&mut self) -> u64 {
        let a16 = self.word();
        self.mmu.write(self.registers.a, a16);
        16
    }

    // 0xEE - XOR d8
    fn xor_d8_0xee(&mut self) -> u64 {
        let d8 = self.byte();
        self.xor(d8);
        8
    }

    // 0xEF - RST 28H
    fn rst_28h_0xef(&mut self) -> u64 {
        self.rst(0x28);
        16
    }
}
