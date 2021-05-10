use super::z80::{Z80};


// Struct representing one instruction
pub struct Opcode {
    pub size: u8,                               // size in bytes of the opcode
    pub instruction: fn(&mut Z80) -> u8         // closure representing the actual opcode steps
                                                //  returns number of clock cycles taken
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

            // 0x2X
            0x20 => Opcode::jr_nz_s8(),
            0x21 => Opcode::ld_hl_d16(),
            0x22 => Opcode::ld_hlp_a(),
            0x23 => Opcode::inc_hl(),
            0x24 => Opcode::inc_h(),
            0x25 => Opcode::dec_h(),
            0x26 => Opcode::ld_h_d8(),
            0x27 => Opcode::daa(),
            0x28 => Opcode::jr_z_s8(),
            0x29 => Opcode::add_hl_hl(),
            0x2A => Opcode::ld_a_hlp(),
            0x2B => Opcode::dec_hl(),
            0x2C => Opcode::inc_l(),
            0x2D => Opcode::dec_l(),
            0x2E => Opcode::ld_l_d8(),
            0x2F => Opcode::cpl(),

            // 0x4X
            0x40 => Opcode::ld_b_b(),
            0x41 => Opcode::ld_b_c(),
            0x42 => Opcode::ld_b_d(),
            0x43 => Opcode::ld_b_e(),
            0x44 => Opcode::ld_b_h(),
            0x45 => Opcode::ld_b_l(),
            0x46 => Opcode::ld_b_hl(),
            0x47 => Opcode::ld_b_a(),
            0x48 => Opcode::ld_c_b(),
            0x49 => Opcode::ld_c_c(),
            0x5A => Opcode::ld_c_d(),
            0x5B => Opcode::ld_c_e(),
            0x5C => Opcode::ld_c_h(),
            0x5D => Opcode::ld_c_l(),
            0x5E => Opcode::ld_c_hl(),
            0x5F => Opcode::ld_c_a(),

            // 0x6X
            0x60 => Opcode::ld_h_b(),
            0x61 => Opcode::ld_h_c(),
            0x62 => Opcode::ld_h_d(),
            0x63 => Opcode::ld_h_e(),
            0x64 => Opcode::ld_h_h(),
            0x65 => Opcode::ld_h_l(),
            0x66 => Opcode::ld_h_hl(),
            0x67 => Opcode::ld_h_a(),
            0x68 => Opcode::ld_l_b(),
            0x69 => Opcode::ld_l_c(),
            0x6A => Opcode::ld_l_d(),
            0x6B => Opcode::ld_l_e(),
            0x6C => Opcode::ld_l_h(),
            0x6D => Opcode::ld_l_l(),
            0x6E => Opcode::ld_l_hl(),
            0x6F => Opcode::ld_l_a(),

            // 0x8X
            0x80 => Opcode::add_a_b(),
            0x81 => Opcode::add_a_c(),
            0x82 => Opcode::add_a_d(),
            0x83 => Opcode::add_a_e(),
            0x84 => Opcode::add_a_h(),
            0x85 => Opcode::add_a_l(),
            0x86 => Opcode::add_a_hl(),
            0x87 => Opcode::add_a_a(),
            0x88 => Opcode::adc_a_b(),
            0x89 => Opcode::adc_a_c(),
            0x8A => Opcode::adc_a_d(),
            0x8B => Opcode::adc_a_e(),
            0x8C => Opcode::adc_a_h(),
            0x8D => Opcode::adc_a_l(),
            0x8E => Opcode::adc_a_hl(),
            0x8F => Opcode::adc_a_a(),


            _ => panic!("Unmapped opcode {}", code)
        }
    }

    // 0x00 - NOP
    fn nop() -> Opcode {
        Opcode {
            size: 1,
            instruction: |_cpu: &mut Z80| { 4 }
        }
    }

    // 0x01 - LD BC, d16
    fn ld_bc() -> Opcode {
        Opcode {
            size: 3,
            instruction: |cpu: &mut Z80| {
                cpu.registers.c = cpu.byte();
                cpu.registers.b = cpu.byte();
                12
            }
        }
    }

    // 0x02 - LD (BC), A
    fn ld_bc_a() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.mmu.write(cpu.registers.a, Z80::u16_from_u8(cpu.registers.b, cpu.registers.c));
                8
            }
        }
    }

    // 0x03 - INC BC
    fn inc_bc() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                let bc = cpu.get_bc();
                let bc = cpu.add_16(bc, 1, false);
                cpu.set_bc(bc);
                8
            }
        }
    }

    // 0x04 - INC B
    fn inc_b() -> Opcode {
        Opcode{
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.b = cpu.add_8(cpu.registers.b, 1, true);
                4
            }
        }
    }

    // 0x05 - DEC b
    fn dec_b() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.b = cpu.sub_8(cpu.registers.b, 1, true);
                4
            }
        }
    }

    // 0x06 - LD B, d8
    fn ld_b() -> Opcode {
        Opcode {
            size: 2,
            instruction: |cpu: &mut Z80| {
                cpu.registers.b = cpu.byte();
                8
            }
        }
    }

    // 0x07 - RLCA
    fn rlca() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.a = (cpu.registers.a << 1) | (cpu.registers.a >> 7);
                cpu.unset_zero();
                cpu.unset_subtraction();
                cpu.unset_half_carry();

                match cpu.registers.a & 0x80 {
                    0x80 => cpu.set_full_carry(),
                    _ => cpu.unset_full_carry()
                };
                4
            }
        }
    }

    // 0x08 - LD (a16), SP
    fn ld_a16_sp() -> Opcode {
        Opcode {
            size: 3,
            instruction: |cpu: &mut Z80| {
                let addr_lower = cpu.byte();
                let addr_upper = cpu.byte();
                let addr = Z80::u16_from_u8 (addr_upper, addr_lower);
                cpu.mmu.write(cpu.registers.sp as u8, addr);
                cpu.mmu.write((cpu.registers.sp >> 8) as u8, addr);
                20
            }
        }
    }

    // 0x09 - ADD HL, BC
    fn add_hl_bc() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                let hl = cpu.get_hl();
                let bc = cpu.get_bc();
                let hl = cpu.add_16(hl, bc, true);
                cpu.set_hl(hl);
                8
            }
        }
    }

    // 0x0A - LD A, (BC)
    fn ld_a_bc() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.a = cpu.mmu.read(Z80::u16_from_u8(cpu.registers.b, cpu.registers.c));
                8
            }
        }
    }

    // 0x0B - DEC BC
    fn dec_bc() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                let bc = cpu.get_bc();
                let bc = cpu.sub_16(bc, 1, false);
                cpu.set_bc(bc);
                8
            }
        }
    }

    // 0x0C - INC C
    fn inc_c() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.c = cpu.inc_8(cpu.registers.c, true);
                4
            }
        }
    }

    // 0x0D - DEC C
    fn dec_c() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.c = cpu.dec_8(cpu.registers.c, true);
                4
            }
        }
    }

    // 0x0E - LD C, d8
    fn ld_c() -> Opcode {
        Opcode {
            size: 2,
            instruction: |cpu: &mut Z80| {
                cpu.registers.c = cpu.byte();
                8
            }
        }
    }

    // 0x0F - RRCA
    fn rrca() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.a = (cpu.registers.a >> 1) | (cpu.registers.a << 7);
                cpu.unset_zero();
                cpu.unset_subtraction();
                cpu.unset_half_carry();
                match cpu.registers.a >> 7 != 0{
                    true => cpu.set_full_carry(),
                    false => cpu.unset_full_carry()
                };
                4
            }
        }
    }





    // 0x20 - JR NZ s8
    fn jr_nz_s8() -> Opcode {
        Opcode {
            size: 2,
            instruction: |cpu: &mut Z80| {
                match cpu.is_zero() {
                    true => {
                        let next = cpu.byte() as i8;
                        let conv = (cpu.registers.pc as u32 as i32) + (next as i32);
                        cpu.registers.pc = conv as u16;
                        12
                    },
                    false => {
                        cpu.registers.pc += 2;
                        8
                    }
                }
            }
        }
    }

    // 0x21 - LD HL d16
    fn ld_hl_d16() -> Opcode {
        Opcode {
            size: 3,
            instruction: |cpu: &mut Z80| {
                cpu.registers.h = cpu.byte();
                cpu.registers.l = cpu.byte();
                12
            }
        }
    }

    // 0x22 - LD (HL+) A
    fn ld_hlp_a() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                let hl = cpu.get_hl();
                cpu.mmu.write(cpu.registers.a, hl);
                let hl = cpu.inc_16(hl, false);
                cpu.set_hl(hl);
                8
            }
        }
    }

    // 0x23 - INC HL
    fn inc_hl() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                let hl = cpu.get_hl();
                let hl = cpu.inc_16(hl, false);
                cpu.set_hl(hl);
                8
            }
        }
    }

    // 0x24 - INC H
    fn inc_h() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                let h = cpu.registers.h;
                let h = cpu.inc_8(h, true);
                cpu.registers.h = h;
                4
            }
        }
    }

    // 0x25 - DEC H
    fn dec_h() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                let h = cpu.registers.h;
                let h = cpu.dec_8(h, true);
                cpu.registers.h = h;
                4
            }
        }
    }

    // 0x26 - LD H d8
    fn ld_h_d8() -> Opcode {
        Opcode {
            size: 2,
            instruction: |cpu: &mut Z80| {
                cpu.registers.h = cpu.byte();
                8
            }
        }
    }

    // 0x27 - DAA
    fn daa() -> Opcode {
       Opcode {
           size: 1,
           instruction: |cpu: &mut Z80| {
               cpu.daa();
               4
           }
       }
    }

    // 0x28 - JR Z s8
    fn jr_z_s8() -> Opcode {
        Opcode {
            size: 0,    // Real: 2 bytes, but directly modified in instruction
            instruction: |cpu: &mut Z80| {
                match cpu.is_zero() {
                    true => {
                        cpu.registers.pc += 2;
                        8
                    },

                    false => {
                        let next = cpu.byte() as i8;
                        let conv = (cpu.registers.pc as u32 as i32) + (next as i32);
                        cpu.registers.pc = conv as u16;
                        12
                    }
                }
            }
        }
    }

    // 0x29 - ADD HL HL
    fn add_hl_hl() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                let hl = cpu.get_hl();
                let hl = cpu.add_16(hl, hl, true);
                cpu.set_hl(hl);
                8
            }
        }
    }

    // 0x2A LD A HL+
    fn ld_a_hlp () -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                let hl = cpu.get_hl();
                cpu.registers.a = cpu.mmu.read(hl);
                let hl = cpu.inc_16(hl, false);
                cpu.set_hl(hl);
                8
            }
        }
    }

    // 0x2B - DEC HL
    fn dec_hl() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                let hl = cpu.get_hl();
                let hl = cpu.dec_16(hl, false);
                cpu.set_hl(hl);
                8
            }
        }
    }

    // 0x2C - INC L
    fn inc_l() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                let l = cpu.registers.l;
                let l = cpu.inc_8(l, true);
                cpu.registers.l = l;
                4
            }
        }
    }

    // 0x2D - DEC L
    fn dec_l() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                let l = cpu.registers.l;
                let l = cpu.dec_8(l, true);
                cpu.registers.l = l;
                4
            }
        }
    }

    // 0x2E - LD L d8
    fn ld_l_d8() -> Opcode {
        Opcode {
            size: 2,
            instruction: |cpu: &mut Z80| {
                cpu.registers.l = cpu.byte();
                8
            }
        }
    }

    // 0x2F - CPL
    fn cpl() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.a = !cpu.registers.a;
                4
            }
        }
    }




    // 0x40 - LD B B
    fn ld_b_b() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.b = cpu.registers.b;      // ah, yes
                4
            }
        }
    }

    // 0x41 - LD B C
    fn ld_b_c() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.b = cpu.registers.c;
                4
            }
        }
    }

    // 0x42 - LD B D
    fn ld_b_d() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.b = cpu.registers.d;
                4
            }
        }
    }

    // 0x43 - LD B E
    fn ld_b_e() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.b = cpu.registers.e;
                4
            }
        }
    }

    // 0x44 - LD B H
    fn ld_b_h() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.b = cpu.registers.h;
                4
            }
        }
    }

    // 0x45 - LD B L
    fn ld_b_l() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.b = cpu.registers.l;
                4
            }
        }
    }

    // 0x46 - LD B (HL)
    fn ld_b_hl() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.b = cpu.byte();
                8
            }
        }
    }

    // 0x47 - LD B A
    fn ld_b_a() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.b = cpu.registers.a;
                4
            }
        }
    }

    // 0x48 - LD C B
    fn ld_c_b() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.c = cpu.registers.b;
                4
            }
        }
    }

    // 0x49 - LD C C
    fn ld_c_c() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.c = cpu.registers.c;      // ok
                4
            }
        }
    }

    // 0x4A - LD C D
    fn ld_c_d() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.c = cpu.registers.d;
                4
            }
        }
    }

    // 0x4B - LD C E
    fn ld_c_e() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.c = cpu.registers.e;
                4
            }
        }
    }

    // 0x4C - LD C H
    fn ld_c_h() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.c = cpu.registers.h;
                4
            }
        }
    }

    // 0x4D - LD C L
    fn ld_c_l() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.c = cpu.registers.l;
                4
            }
        }
    }

    // 0x4E - LD C (HL)
    fn ld_c_hl() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.c = cpu.mmu.read(cpu.get_hl());
                8
            }
        }
    }

    // 0x4F - LD C A
    fn ld_c_a() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.c = cpu.registers.a;
                4
            }
        }
    }




    // 0x60 - LD H B
    fn ld_h_b() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.h = cpu.registers.b;
                4
            }
        }
    }

    // 0x61 - LD H C
    fn ld_h_c() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.h = cpu.registers.c;
                4
            }
        }
    }

    // 0x62 - LD H D
    fn ld_h_d() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.h = cpu.registers.d;
                4
            }
        }
    }

    // 0x63 - LD H E
    fn ld_h_e() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.h = cpu.registers.e;
                4
            }
        }
    }

    // 0x64 - LD H H
    fn ld_h_h() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.h = cpu.registers.h;      // sure
                4
            }
        }
    }

    // 0x65 - LD H L
    fn ld_h_l() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.h = cpu.registers.l;
                4
            }
        }
    }

    // 0x66 - LD H (HL)
    fn ld_h_hl() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.h = cpu.byte();
                8
            }
        }
    }

    // 0x67 - LD H A
    fn ld_h_a() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.h = cpu.registers.a;
                4
            }
        }
    }

    // 0x68 - LD L B
    fn ld_l_b() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.l = cpu.registers.b;
                4
            }
        }
    }

    // 0x69 - LD L C
    fn ld_l_c() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.l = cpu.registers.c;
                4
            }
        }
    }

    // 0x6A - LD L D
    fn ld_l_d() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.l = cpu.registers.d;
                4
            }
        }
    }

    // 0x6B - LD L E
    fn ld_l_e() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.l = cpu.registers.e;
                4
            }
        }
    }

    // 0x6C - LD L H
    fn ld_l_h() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.l = cpu.registers.h;
                4
            }
        }
    }

    // 0x6D - LD L L
    fn ld_l_l() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.l = cpu.registers.l;      // ok
                4
            }
        }
    }

    // 0x6E - LD (HL)
    fn ld_l_hl() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.l = cpu.mmu.read(cpu.get_hl());
                8
            }
        }
    }

    // 0x6F - LD L A
    fn ld_l_a() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.l = cpu.registers.a;
                4
            }
        }
    }




    // 0x80 - ADD A,B
    fn add_a_b() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.a = cpu.add_8(cpu.registers.a, cpu.registers.b, true);
                4
            }
        }
    }

    // 0x81 - ADD A,C
    fn add_a_c() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.a = cpu.add_8(cpu.registers.a, cpu.registers.c, true);
                4
            }
        }
    }

    // 0x82 - ADD A,D
    fn add_a_d() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.a = cpu.add_8(cpu.registers.a, cpu.registers.d, true);
                4
            }
        }
    }

    // 0x83 - ADD A,E
    fn add_a_e() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.a = cpu.add_8(cpu.registers.a, cpu.registers.e, true);
                4
            }
        }
    }

    // 0x84 - ADD A,H
    fn add_a_h() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.a = cpu.add_8(cpu.registers.a, cpu.registers.h, true);
                4
            }
        }
    }

    // 0x85 - ADD A,L
    fn add_a_l() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.a = cpu.add_8(cpu.registers.a, cpu.registers.l, true);
                4
            }
        }
    }

    // 0x86 - ADD A,(HL)
    fn add_a_hl() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                let value = cpu.mmu.read(cpu.get_hl());
                cpu.registers.a = cpu.add_8(cpu.registers.a, value, true);
                8
            }
        }
    }

    // 0x87 - ADD A,A
    fn add_a_a() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.a = cpu.add_8(cpu.registers.a, cpu.registers.a, true);
                4
            }
        }
    }

    // 0x88 - ADC A,B
    fn adc_a_b() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.a = cpu.adc_8(cpu.registers.a, cpu.registers.b);
                4
            }
        }
    }

    // 0x89 - ADC A,C
    fn adc_a_c() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.a = cpu.adc_8(cpu.registers.a, cpu.registers.c);
                4
            }
        }
    }

    // 0x8A - ADC A,D
    fn adc_a_d() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.a = cpu.adc_8(cpu.registers.a, cpu.registers.d);
                4
            }
        }
    }

    // 0x8B - ADC A,E
    fn adc_a_e() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.a = cpu.adc_8(cpu.registers.a, cpu.registers.e);
                4
            }
        }
    }

    // 0x8C - ADC A,H
    fn adc_a_h() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.a = cpu.adc_8(cpu.registers.a, cpu.registers.h);
                4
            }
        }
    }

    // 0x8D - ADC A,L
    fn adc_a_l() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.a = cpu.adc_8(cpu.registers.a, cpu.registers.l);
                4
            }
        }
    }

    // 0x8E - ADC A,(HL)
    fn adc_a_hl() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                let value = cpu.mmu.read(cpu.get_hl());
                cpu.registers.a = cpu.adc_8(cpu.registers.a, value);
                8
            }
        }
    }

    // 0x8F - ADC A,A
    fn adc_a_a() -> Opcode {
        Opcode {
            size: 1,
            instruction: |cpu: &mut Z80| {
                cpu.registers.a = cpu.adc_8(cpu.registers.a, cpu.registers.a);
                4
            }
        }
    }
}
