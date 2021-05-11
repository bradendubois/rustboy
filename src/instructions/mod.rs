use super::z80::{Z80};



impl Z80 {

    pub fn lookup(&mut self, code: u8) -> u64 {
        match code {

            // 0x0X
            0x00 => Z80::nop(self),
            0x01 => Z80::ld_bc(self),
            0x02 => Z80::ld_bc_a(self),
            0x03 => Z80::inc_bc(self),
            0x04 => Z80::inc_b(self),
            0x05 => Z80::dec_b(self),
            0x06 => Z80::ld_b(self),
            0x07 => Z80::rlca(self),
            0x08 => Z80::ld_a16_sp(self),
            0x09 => Z80::add_hl_bc(self),
            0x0A => Z80::ld_a_bc(self),
            0x0B => Z80::dec_bc(self),
            0x0C => Z80::inc_c(self),
            0x0D => Z80::dec_c(self),
            0x0E => Z80::ld_c(self),
            0x0F => Z80::rrca(self),

            // 0x2X
            0x20 => Z80::jr_nz_s8(self),
            0x21 => Z80::ld_hl_d16(self),
            0x22 => Z80::ld_hlp_a(self),
            0x23 => Z80::inc_hl(self),
            0x24 => Z80::inc_h(self),
            0x25 => Z80::dec_h(self),
            0x26 => Z80::ld_h_d8(self),
            0x27 => Z80::daa(self),
            0x28 => Z80::jr_z_s8(self),
            0x29 => Z80::add_hl_hl(self),
            0x2A => Z80::ld_a_hlp(self),
            0x2B => Z80::dec_hl(self),
            0x2C => Z80::inc_l(self),
            0x2D => Z80::dec_l(self),
            0x2E => Z80::ld_l_d8(self),
            0x2F => Z80::cpl(self),

            // 0x4X
            0x40 => Z80::ld_b_b(self),
            0x41 => Z80::ld_b_c(self),
            0x42 => Z80::ld_b_d(self),
            0x43 => Z80::ld_b_e(self),
            0x44 => Z80::ld_b_h(self),
            0x45 => Z80::ld_b_l(self),
            0x46 => Z80::ld_b_hl(self),
            0x47 => Z80::ld_b_a(self),
            0x48 => Z80::ld_c_b(self),
            0x49 => Z80::ld_c_c(self),
            0x5A => Z80::ld_c_d(self),
            0x5B => Z80::ld_c_e(self),
            0x5C => Z80::ld_c_h(self),
            0x5D => Z80::ld_c_l(self),
            0x5E => Z80::ld_c_hl(self),
            0x5F => Z80::ld_c_a(self),

            // 0x6X
            0x60 => Z80::ld_h_b(self),
            0x61 => Z80::ld_h_c(self),
            0x62 => Z80::ld_h_d(self),
            0x63 => Z80::ld_h_e(self),
            0x64 => Z80::ld_h_h(self),
            0x65 => Z80::ld_h_l(self),
            0x66 => Z80::ld_h_hl(self),
            0x67 => Z80::ld_h_a(self),
            0x68 => Z80::ld_l_b(self),
            0x69 => Z80::ld_l_c(self),
            0x6A => Z80::ld_l_d(self),
            0x6B => Z80::ld_l_e(self),
            0x6C => Z80::ld_l_h(self),
            0x6D => Z80::ld_l_l(self),
            0x6E => Z80::ld_l_hl(self),
            0x6F => Z80::ld_l_a(self),

            // 0x8X
            0x80 => Z80::add_a_b(self),
            0x81 => Z80::add_a_c(self),
            0x82 => Z80::add_a_d(self),
            0x83 => Z80::add_a_e(self),
            0x84 => Z80::add_a_h(self),
            0x85 => Z80::add_a_l(self),
            0x86 => Z80::add_a_hl(self),
            0x87 => Z80::add_a_a(self),
            0x88 => Z80::adc_a_b(self),
            0x89 => Z80::adc_a_c(self),
            0x8A => Z80::adc_a_d(self),
            0x8B => Z80::adc_a_e(self),
            0x8C => Z80::adc_a_h(self),
            0x8D => Z80::adc_a_l(self),
            0x8E => Z80::adc_a_hl(self),
            0x8F => Z80::adc_a_a(self),

            // 0xAX
            0xA0 => Z80::and_b(self),
            0xA1 => Z80::and_c(self),
            0xA2 => Z80::and_d(self),
            0xA3 => Z80::and_e(self),
            0xA4 => Z80::and_h(self),
            0xA5 => Z80::and_l(self),
            0xA6 => Z80::and_hl(self),
            0xA7 => Z80::and_a(self),
            0xA8 => Z80::xor_b(self),
            0xA9 => Z80::xor_c(self),
            0xAA => Z80::xor_d(self),
            0xAB => Z80::xor_e(self),
            0xAC => Z80::xor_h(self),
            0xAD => Z80::xor_l(self),
            0xAE => Z80::xor_hl(self),
            0xAF => Z80::xor_a(self),

            // 0xCX
            0xC0 => Z80::ret_nz(self),
            0xC1 => Z80::pop_bc(self),
            0xC2 => Z80::jp_nz_a16(self),
            0xC3 => Z80::jp_a16(self),
            0xC4 => Z80::call_nz_a16(self),
            0xC5 => Z80::push_bc(self),
            0xC6 => Z80::add_a_d8(self),
            0xC7 => Z80::add_rst_00h(self),
            0xC8 => Z80::ret_z(self),
            0xC9 => Z80::ret(self),
            0xCA => Z80::jp_z_a16(self),
            0xCB => Z80::cb(self),
            0xCC => Z80::call_z_a16(self),
            0xCD => Z80::call_a16(self),
            0xCE => Z80::call_adc_a_d8(self),
            0xCF => Z80::add_rst_08h(self),

            // 0xEX
            0xE0 => Z80::ldh_a8_a(self),
            0xE1 => Z80::pop_hl(self),
            0xE2 => Z80::ld_c_a_ram(self),
            0xE5 => Z80::push_hl(self),
            0xE6 => Z80::and_d8(self),
            0xE7 => Z80::add_rst_20h(self),
            0xE8 => Z80::add_sp_r8(self),
            0xE9 => Z80::jp_hl(self),
            0xEA => Z80::ld_a16_a(self),
            0xEE => Z80::xor_d8(self),
            0xEF => Z80::add_rst_28h(self),


            _ => panic!("Unmapped opcode {}", code)
        }
    }

    // 0x00 - NOP
    fn nop(&mut self) -> u64 {
        4
    }

    // 0x01 - LD BC, d16
    fn ld_bc(&mut self) -> u64 {
        self.registers.c = self.byte();
        self.registers.b = self.byte();
        12
    }

    // 0x02 - LD (BC), A
    fn ld_bc_a(&mut self) -> u64 {
        self.mmu.write(self.registers.a, self.get_bc());
        8
    }

    // 0x03 - INC BC
    fn inc_bc(&mut self) -> u64 {
        let bc = self.get_bc();
        let bc = self.add_16(bc, 1, false);
        self.set_bc(bc);
        8
    }

    // 0x04 - INC B
    fn inc_b(&mut self) -> u64 {
        self.registers.b = self.add_8(self.registers.b, 1, true);
        4
    }

    // 0x05 - DEC b
    fn dec_b(&mut self) -> u64 {
        self.registers.b = self.sub_8(self.registers.b, 1, true);
        4
    }

    // 0x06 - LD B, d8
    fn ld_b(&mut self) -> u64 {
        self.registers.b = self.byte();
        8
    }

    // 0x07 - RLCA
    fn rlca(&mut self) -> u64 {
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
    fn ld_a16_sp(&mut self) -> u64 {
        let addr = self.word();
        self.mmu.write(self.registers.sp as u8, addr);
        self.mmu.write((self.registers.sp >> 8) as u8, addr + 1);
        20
    }

    // 0x09 - ADD HL, BC
    fn add_hl_bc(&mut self) -> u64 {
        let hl = self.get_hl();
        let bc = self.get_bc();
        let hl = self.add_16(hl, bc, true);
        self.set_hl(hl);
        8
    }

    // 0x0A - LD A, (BC)
    fn ld_a_bc(&mut self) -> u64 {
        self.registers.a = self.mmu.read(self.get_bc());
        8
    }

    // 0x0B - DEC BC
    fn dec_bc(&mut self) -> u64 {
        let bc = self.get_bc();
        let bc = self.sub_16(bc, 1, false);
        self.set_bc(bc);
        8
    }

    // 0x0C - INC C
    fn inc_c(&mut self) -> u64 {
        self.registers.c = self.inc_8(self.registers.c, true);
        4
    }

    // 0x0D - DEC C
    fn dec_c(&mut self) -> u64 {
        self.registers.c = self.dec_8(self.registers.c, true);
        4
    }

    // 0x0E - LD C, d8
    fn ld_c(&mut self) -> u64 {
        self.registers.c = self.byte();
        8
    }

    // 0x0F - RRCA
    fn rrca(&mut self) -> u64 {
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

    // 0x20 - JR NZ s8
    fn jr_nz_s8(&mut self) -> u64 {
        match self.is_zero() {
            true => {
                let next = self.byte() as i8;
                let conv = (self.registers.pc as u32 as i32) + (next as i32);
                self.registers.pc = conv as u16;
                12
            },
            false => {
                self.registers.pc += 2;
                8
            }
        }
    }

    // 0x21 - LD HL d16
    fn ld_hl_d16(&mut self) -> u64 {
        self.registers.h = self.byte();
        self.registers.l = self.byte();
        12
    }

    // 0x22 - LD (HL+) A
    fn ld_hlp_a(&mut self) -> u64 {
        let hl = self.get_hl();
        self.mmu.write(self.registers.a, hl);
        let hl = self.inc_16(hl, false);
        self.set_hl(hl);
        8
    }

    // 0x23 - INC HL
    fn inc_hl(&mut self) -> u64 {
        let hl = self.get_hl();
        let hl = self.inc_16(hl, false);
        self.set_hl(hl);
        8
    }

    // 0x24 - INC H
    fn inc_h(&mut self) -> u64 {
        let h = self.registers.h;
        let h = self.inc_8(h, true);
        self.registers.h = h;
        4
    }

    // 0x25 - DEC H
    fn dec_h(&mut self) -> u64 {
        let h = self.registers.h;
        let h = self.dec_8(h, true);
        self.registers.h = h;
        4
    }

    // 0x26 - LD H d8
    fn ld_h_d8(&mut self) -> u64 {
        self.registers.h = self.byte();
        8
    }

    // 0x27 - DAA
    fn daa(&mut self) -> u64 {
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
        4
    }

    // 0x28 - JR Z s8
    fn jr_z_s8(&mut self) -> u64 {
        match self.is_zero() {
            true => {
                self.registers.pc += 2;
                8
            },

            false => {
                let next = self.byte() as i8;
                let conv = (self.registers.pc as u32 as i32) + (next as i32);
                self.registers.pc = conv as u16;
                12
            }
        }
    }

    // 0x29 - ADD HL HL
    fn add_hl_hl(&mut self) -> u64 {
        let hl = self.get_hl();
        let hl = self.add_16(hl, hl, true);
        self.set_hl(hl);
        8
    }

    // 0x2A LD A HL+
    fn ld_a_hlp (&mut self) -> u64 {
        let hl = self.get_hl();
        self.registers.a = self.mmu.read(hl);
        let hl = self.inc_16(hl, false);
        self.set_hl(hl);
        8
    }

    // 0x2B - DEC HL
    fn dec_hl(&mut self) -> u64 {
        let hl = self.get_hl();
        let hl = self.dec_16(hl, false);
        self.set_hl(hl);
        8
    }

    // 0x2C - INC L
    fn inc_l(&mut self) -> u64 {
        let l = self.registers.l;
        let l = self.inc_8(l, true);
        self.registers.l = l;
        4
    }

    // 0x2D - DEC L
    fn dec_l(&mut self) -> u64 {
        let l = self.registers.l;
        let l = self.dec_8(l, true);
        self.registers.l = l;
        4
    }

    // 0x2E - LD L d8
    fn ld_l_d8(&mut self) -> u64 {
        self.registers.l = self.byte();
        8
    }

    // 0x2F - CPL
    fn cpl(&mut self) -> u64 {
        self.registers.a = !self.registers.a;
        4
    }

    // 0x40 - LD B B
    fn ld_b_b(&mut self) -> u64 {
        self.registers.b = self.registers.b;      // ah, yes
        4
    }

    // 0x41 - LD B C
    fn ld_b_c(&mut self) -> u64 {
        self.registers.b = self.registers.c;
        4
    }

    // 0x42 - LD B D
    fn ld_b_d(&mut self) -> u64 {
        self.registers.b = self.registers.d;
        4
    }

    // 0x43 - LD B E
    fn ld_b_e(&mut self) -> u64 {
        self.registers.b = self.registers.e;
        4
    }

    // 0x44 - LD B H
    fn ld_b_h(&mut self) -> u64 {
        self.registers.b = self.registers.h;
        4
    }

    // 0x45 - LD B L
    fn ld_b_l(&mut self) -> u64 {
        self.registers.b = self.registers.l;
        4
    }

    // 0x46 - LD B (HL)
    fn ld_b_hl(&mut self) -> u64 {
        self.registers.b = self.byte();
        8
    }

    // 0x47 - LD B A
    fn ld_b_a(&mut self) -> u64 {
        self.registers.b = self.registers.a;
        4
    }

    // 0x48 - LD C B
    fn ld_c_b(&mut self) -> u64 {
        self.registers.c = self.registers.b;
        4
    }

    // 0x49 - LD C C
    fn ld_c_c(&mut self) -> u64 {
        self.registers.c = self.registers.c;      // ok
        4
    }

    // 0x4A - LD C D
    fn ld_c_d(&mut self) -> u64 {
        self.registers.c = self.registers.d;
        4
    }

    // 0x4B - LD C E
    fn ld_c_e(&mut self) -> u64 {
        self.registers.c = self.registers.e;
        4
    }

    // 0x4C - LD C H
    fn ld_c_h(&mut self) -> u64 {
        self.registers.c = self.registers.h;
        4
    }

    // 0x4D - LD C L
    fn ld_c_l(&mut self) -> u64 {
        self.registers.c = self.registers.l;
        4
    }

    // 0x4E - LD C (HL)
    fn ld_c_hl(&mut self) -> u64 {
        self.registers.c = self.mmu.read(self.get_hl());
        8
    }

    // 0x4F - LD C A
    fn ld_c_a(&mut self) -> u64 {
        self.registers.c = self.registers.a;
        4
    }

    // 0x60 - LD H B
    fn ld_h_b(&mut self) -> u64 {
        self.registers.h = self.registers.b;
        4
    }

    // 0x61 - LD H C
    fn ld_h_c(&mut self) -> u64 {
        self.registers.h = self.registers.c;
        4
    }

    // 0x62 - LD H D
    fn ld_h_d(&mut self) -> u64 {
        self.registers.h = self.registers.d;
        4
    }

    // 0x63 - LD H E
    fn ld_h_e(&mut self) -> u64 {
        self.registers.h = self.registers.e;
        4
    }

    // 0x64 - LD H H
    fn ld_h_h(&mut self) -> u64 {
        self.registers.h = self.registers.h;      // sure
        4
    }

    // 0x65 - LD H L
    fn ld_h_l(&mut self) -> u64 {
        self.registers.h = self.registers.l;
        4
    }

    // 0x66 - LD H (HL)
    fn ld_h_hl(&mut self) -> u64 {
        self.registers.h = self.byte();
        8
    }

    // 0x67 - LD H A
    fn ld_h_a(&mut self) -> u64 {
        self.registers.h = self.registers.a;
        4
    }

    // 0x68 - LD L B
    fn ld_l_b(&mut self) -> u64 {
        self.registers.l = self.registers.b;
        4
    }

    // 0x69 - LD L C
    fn ld_l_c(&mut self) -> u64 {
        self.registers.l = self.registers.c;
        4
    }

    // 0x6A - LD L D
    fn ld_l_d(&mut self) -> u64 {
        self.registers.l = self.registers.d;
        4
    }

    // 0x6B - LD L E
    fn ld_l_e(&mut self) -> u64 {
        self.registers.l = self.registers.e;
        4
    }

    // 0x6C - LD L H
    fn ld_l_h(&mut self) -> u64 {
        self.registers.l = self.registers.h;
        4
    }

    // 0x6D - LD L L
    fn ld_l_l(&mut self) -> u64 {
        self.registers.l = self.registers.l;      // ok
        4
    }

    // 0x6E - LD (HL)
    fn ld_l_hl(&mut self) -> u64 {
        self.registers.l = self.mmu.read(self.get_hl());
        8
    }

    // 0x6F - LD L A
    fn ld_l_a(&mut self) -> u64 {
        self.registers.l = self.registers.a;
        4
    }

    // 0x80 - ADD A,B
    fn add_a_b(&mut self) -> u64 {
        self.registers.a = self.add_8(self.registers.a, self.registers.b, true);
        4
    }

    // 0x81 - ADD A,C
    fn add_a_c(&mut self) -> u64 {
        self.registers.a = self.add_8(self.registers.a, self.registers.c, true);
        4
    }

    // 0x82 - ADD A,D
    fn add_a_d(&mut self) -> u64 {
        self.registers.a = self.add_8(self.registers.a, self.registers.d, true);
        4
    }

    // 0x83 - ADD A,E
    fn add_a_e(&mut self) -> u64 {
        self.registers.a = self.add_8(self.registers.a, self.registers.e, true);
        4
    }

    // 0x84 - ADD A,H
    fn add_a_h(&mut self) -> u64 {
        self.registers.a = self.add_8(self.registers.a, self.registers.h, true);
        4
    }

    // 0x85 - ADD A,L
    fn add_a_l(&mut self) -> u64 {
        self.registers.a = self.add_8(self.registers.a, self.registers.l, true);
        4
    }

    // 0x86 - ADD A,(HL)
    fn add_a_hl(&mut self) -> u64 {
        let value = self.mmu.read(self.get_hl());
        self.registers.a = self.add_8(self.registers.a, value, true);
        8
    }

    // 0x87 - ADD A,A
    fn add_a_a(&mut self) -> u64 {
        self.registers.a = self.add_8(self.registers.a, self.registers.a, true);
        4
    }

    // 0x88 - ADC A,B
    fn adc_a_b(&mut self) -> u64 {
        self.adc_8(self.registers.b);
        4
    }

    // 0x89 - ADC A,C
    fn adc_a_c(&mut self) -> u64 {
        self.adc_8(self.registers.c);
        4
    }

    // 0x8A - ADC A,D
    fn adc_a_d(&mut self) -> u64 {
        self.adc_8(self.registers.d);
        4
    }

    // 0x8B - ADC A,E
    fn adc_a_e(&mut self) -> u64 {
        self.adc_8(self.registers.e);
        4
    }

    // 0x8C - ADC A,H
    fn adc_a_h(&mut self) -> u64 {
        self.adc_8(self.registers.h);
        4
    }

    // 0x8D - ADC A,L
    fn adc_a_l(&mut self) -> u64 {
        self.adc_8(self.registers.l);
        4
    }

    // 0x8E - ADC A,(HL)
    fn adc_a_hl(&mut self) -> u64 {
        let value = self.mmu.read(self.get_hl());
        self.adc_8(value);
        8
    }

    // 0x8F - ADC A,A
    fn adc_a_a(&mut self) -> u64 {
        self.adc_8(self.registers.a);
        4
    }

    // 0xA0 - AND B
    fn and_b(&mut self) -> u64 {
        self.and(self.registers.b);
        4
    }

    // 0xA1 - AND C
    fn and_c(&mut self) -> u64 {
        self.and(self.registers.c);
        4
    }

    // 0xA2 - AND D
    fn and_d(&mut self) -> u64 {
        self.and(self.registers.d);
        4
    }

    // 0xA3 - AND E
    fn and_e(&mut self) -> u64 {
        self.and(self.registers.e);
        4
    }

    // 0xA4 - AND H
    fn and_h(&mut self) -> u64 {
        self.and(self.registers.h);
        4
    }

    // 0xA5 - AND L
    fn and_l(&mut self) -> u64 {
        self.and(self.registers.l);
        4
    }

    // 0xA6 - AND (HL)
    fn and_hl(&mut self) -> u64 {
        let value = self.mmu.read(self.get_hl());
        self.and(value);
        8
    }

    // 0xA7 - AND A
    fn and_a(&mut self) -> u64 {
        self.and(self.registers.a);   // ok
        4
    }

    // 0xA8 - XOR B
    fn xor_b(&mut self) -> u64 {
        self.xor(self.registers.b);
        4
    }

    // 0xA9 - XOR C
    fn xor_c(&mut self) -> u64 {
        self.xor(self.registers.c);
        4
    }

    // 0xAA - XOR D
    fn xor_d(&mut self) -> u64 {
        self.xor(self.registers.d);
        4
    }

    // 0xAB - XOR E
    fn xor_e(&mut self) -> u64 {
        self.xor(self.registers.e);
        4
    }

    // 0xAC - XOR H
    fn xor_h(&mut self) -> u64 {
        self.xor(self.registers.h);
        4
    }

    // 0xAD - XOR L
    fn xor_l(&mut self) -> u64 {
        self.xor(self.registers.l);
        4
    }

    // 0xAE - XOR (HL)
    fn xor_hl(&mut self) -> u64 {
        let value = self.mmu.read(self.get_hl());
        self.xor(value);
        8
    }

    // 0xAF - XOR A
    fn xor_a(&mut self) -> u64 {
        self.xor(self.registers.a);   // why not
        4
    }

    // 0xC0 - RET NZ
    fn ret_nz(&mut self) -> u64 {
        match self.is_zero() {
            true => 8,
            false => {
                self.registers.pc = self.pop_sp();
                20
            }
        }
    }

    // 0xC1 - POP BC
    fn pop_bc(&mut self) -> u64 {
        let value = self.pop_sp();
        self.set_bc(value);
        12
    }

    // 0xC2 - JP NZ a16
    fn jp_nz_a16(&mut self) -> u64 {
        let value = self.word();
        match self.is_zero() {
            false => {
                self.registers.pc = value;
                4
            },
            true => 3
        }
    }

    // 0xC3 - JP a16
    fn jp_a16(&mut self) -> u64 {
        let value = self.word();
        self.registers.pc = value;
        16
    }

    // 0xC4 - CALL NZ a16
    fn call_nz_a16(&mut self) -> u64 {
        let value = self.word();
        match self.is_zero() {
            true => {
                self.push_sp(value);
                self.registers.pc = value;
                24
            },
            false => 12
        }
    }

    // 0xC5 - PUSH BC
    fn push_bc(&mut self) -> u64 {
        self.push_sp(self.get_bc());
        16
    }

    // 0xC6 - ADD d8
    fn add_a_d8(&mut self) -> u64 {
        let value = self.byte();
        self.registers.a = self.add_8(self.registers.a, value, true);
        8
    }

    // 0xC7 - RST 00H
    fn add_rst_00h(&mut self) -> u64 {
        self.push_sp(self.registers.pc);
        self.registers.pc = 0x00;
        16
    }

    // 0xC8 - RET Z
    fn ret_z(&mut self) -> u64 {
        match self.is_zero() {
            true => {
                let restore = self.pop_sp();
                self.registers.pc = restore;
                20
            },
            false => {
                8
            }
        }
    }

    // 0xC9 - RET
    fn ret(&mut self) -> u64 {
        let restore = self.pop_sp();
        self.registers.pc = restore;
        16
    }

    // 0xCA - JP Z a16
    fn jp_z_a16(&mut self) -> u64 {
        let a16 = self.word();
        match self.is_zero() {
            true => {
                self.registers.pc = a16;
                4
            },
            false => 3
        }
    }

    // 0xCB - CB PREFIX
    fn cb(&mut self) -> u64 {
        // TODO - Figure out how to index a different table from here
        4
    }

    // 0xCC - CALL Z a16
    fn call_z_a16(&mut self) -> u64 {
        let a16 = self.word();
        match self.is_zero() {
            true => {
                self.push_sp(self.registers.pc);
                self.registers.pc = a16;
                6
            },
            false => 3
        }
    }

    // 0xCD - CALL a16
    fn call_a16(&mut self) -> u64 {
        let a16 = self.word();
        self.push_sp(self.registers.pc);
        self.registers.pc = a16;
        6
    }

    // 0xCE - ADC A d8
    fn call_adc_a_d8(&mut self) -> u64 {
        let d8 = self.byte();
        self.adc_8(d8);
        8
    }

    // 0xCF - RST 08H
    fn add_rst_08h(&mut self) -> u64 {
        self.push_sp(self.registers.pc);
        self.registers.pc = 0x08;
        16
    }

    // 0xE0 - LDH (a8) A
    fn ldh_a8_a(&mut self) -> u64 {
        let a8 = self.byte();
        self.mmu.write(self.registers.a, 0xFF00 | a8 as u16);
        12
    }

    // 0xE1 - POP HL
    fn pop_hl(&mut self) -> u64 {
        let value = self.pop_sp();
        self.set_hl(value);
        12
    }

    // 0xE2 - LD (C) A
    fn ld_c_a_ram(&mut self) -> u64 {
        self.mmu.write(self.registers.a, 0xFF00 | self.registers.c as u16);
        8
    }

    // 0xE5 - PUSH HL
    fn push_hl(&mut self) -> u64 {
        self.push_sp(self.get_hl());
        16
    }

    // 0xE6 - AND d8
    fn and_d8(&mut self) -> u64 {
        let d8 = self.byte();
        self.and(d8);
        8
    }

    // 0xE7 - RST 20H
    fn add_rst_20h(&mut self) -> u64 {
        self.push_sp(self.registers.pc);
        self.registers.pc = 0x20;
        16
    }

    // 0xE8 - ADD SP, r8
    fn add_sp_r8(&mut self) -> u64 {
        let r8 = self.byte() as i8 as i16 as u16;
        self.registers.sp = self.add_16(self.registers.sp, r8, true);
        self.unset_zero();
        16
    }

    // 0xE9 - JP (HL)
    fn jp_hl(&mut self) -> u64 {
        self.registers.pc = self.get_hl();
        4
    }

    // 0xEA - LD (a16) A
    fn ld_a16_a(&mut self) -> u64 {
        let a16 = self.word();
        self.mmu.write(self.registers.a, a16);
        16
    }

    // 0xEE - XOR d8
    fn xor_d8(&mut self) -> u64 {
        let d8 = self.byte();
        self.xor(d8);
        8
    }

    // 0xEF - RST 28H
    fn add_rst_28h(&mut self) -> u64 {
        self.push_sp(self.registers.pc);
        self.registers.pc = 0x28;
        16
    }
}
