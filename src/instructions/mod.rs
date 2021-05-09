use super::z80;


// Struct representing one instruction
pub struct Opcode {
    pub size: u16,                      // size in bytes of the opcode; should be 1, 2, 3, no larger
    pub clock_timing: z80::Clock,            // the timing of m and t cycles taken in one instruction
    pub instruction: fn(&mut z80::Z80)       // the actual function that will
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

            _ => panic!("Unmapped opcode {}", code)
        }
    }

    // 0x00 - NOP
    fn nop() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: z80::Clock {
                m: 1,
                t: 4
            },
            instruction: |_cpu: &mut z80::Z80| { }
        }
    }

    // 0x01 - LD BC, d16
    fn ld_bc() -> Opcode {
        Opcode {
            size: 3,
            clock_timing: z80::Clock {
                m: 3,
                t: 12
            },
            instruction: |cpu: &mut z80::Z80| {
                cpu.registers.c = cpu.mmu.read(cpu.registers.pc + 1);
                cpu.registers.b = cpu.mmu.read(cpu.registers.pc + 2);
            }
        }
    }

    // 0x02 - LD (BC), A
    fn ld_bc_a() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: z80::Clock {
                m: 3,
                t: 12
            },
            instruction: |cpu: &mut z80::Z80| {
                cpu.mmu.write(cpu.registers.a, ((cpu.registers.b << 8) + cpu.registers.c).into());
            }
        }
    }

    // 0x03 - INC BC
    fn inc_bc() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: z80::Clock {
                m: 2,
                t: 8
            },
            instruction: |cpu: &mut z80::Z80| {

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
            clock_timing: z80::Clock {
                m: 1,
                t: 4
            },
            instruction: |cpu: &mut z80::Z80| {
                cpu.registers.b = cpu.add_8(cpu.registers.b, 1, true);
            }
        }
    }

    // 0x05 - DEC b
    fn dec_b() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: z80::Clock {
                m: 1,
                t: 4
            },
            instruction: |cpu: &mut z80::Z80| {
                cpu.registers.b = cpu.sub_8(cpu.registers.b, 1, true);
            }
        }
    }

    // 0x06 - LD B, d8
    fn ld_b() -> Opcode {
        Opcode {
            size: 2,
            clock_timing: z80::Clock {
                m: 4,
                t: 8
            },
            instruction: |cpu: &mut z80::Z80| {
                cpu.registers.b = cpu.mmu.read(cpu.registers.pc+1);
            }
        }
    }

    // 0x07 - RLCA
    fn rlca() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: z80::Clock {
                m: 1,
                t: 4
            },
            instruction: |cpu: &mut z80::Z80| {
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
            clock_timing: z80::Clock {
                m: 5,
                t: 15
            },
            instruction: |cpu: &mut z80::Z80| {
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
            clock_timing: z80::Clock {
                m: 2,
                t: 8
            },
            instruction: |cpu: &mut z80::Z80| {

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
            clock_timing: z80::Clock {
                m: 2,
                t: 8
            },
            instruction: |cpu: &mut z80::Z80| {
                cpu.registers.a = cpu.mmu.read(((cpu.registers.b << 8) + cpu.registers.c).into());
            }
        }
    }

    // 0x0B - DEC BC
    fn dec_bc() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: z80::Clock {
                m: 2,
                t: 8
            },
            instruction: |cpu: &mut z80::Z80| {

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
            clock_timing: z80::Clock {
                m: 1,
                t: 4
            },
            instruction: |cpu: &mut z80::Z80| {
                cpu.registers.c = cpu.inc_8(cpu.registers.c, true);
            }
        }
    }

    // 0x0D - DEC C
    fn dec_c() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: z80::Clock {
                m: 1,
                t: 4
            },
            instruction: |cpu: &mut z80::Z80| {
                cpu.registers.c = cpu.dec_8(cpu.registers.c, true);
            }
        }
    }

    // 0x0E - LD C, d8
    fn ld_c() -> Opcode {
        Opcode {
            size: 2,
            clock_timing: z80::Clock {
                m: 2,
                t: 8
            },
            instruction: |cpu: &mut z80::Z80| {
                cpu.registers.c = cpu.mmu.read(cpu.registers.pc+1);
            }
        }
    }

    // 0x0F - RRCA
    fn rrca() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: z80::Clock {
                m: 1,
                t: 4
            },
            instruction: |cpu: &mut z80::Z80| {
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





    // 0x20 - JR NZ s8
    fn jr_nz_s8() -> Opcode {
        Opcode {
            size: 0,    // Real: 2 bytes, but directly modified in instruction
            clock_timing: z80::Clock {
                m: 3,
                t: 2
            },
            instruction: |cpu: &mut z80::Z80| {
                match cpu.is_zero() {
                    true => {
                        let next = cpu.mmu.read(cpu.registers.pc + 1) as i8;
                        let conv = (cpu.registers.pc as u32 as i32) + (next as i32);
                        cpu.registers.pc = conv as u16;
                    },
                    false => cpu.registers.pc += 2,
                }
            }
        }
    }

    // 0x21 - LD HL d16
    fn ld_hl_d16() -> Opcode {
        Opcode {
            size: 3,
            clock_timing: z80::Clock {
                m: 3,
                t: 0
            },
            instruction: |cpu: &mut z80::Z80| {
                cpu.registers.h = cpu.mmu.read(cpu.registers.pc + 2);
                cpu.registers.l = cpu.mmu.read(cpu.registers.pc + 1);
            }
        }
    }

    // 0x22 - LD (HL+) A
    fn ld_hlp_a() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: z80::Clock {
                m: 2,
                t: 8
            },
            instruction: |cpu: &mut z80::Z80| {
                let hl = cpu.get_hl();
                cpu.mmu.write(cpu.registers.a, hl);
                let hl = cpu.inc_16(hl, false);
                cpu.set_hl(hl);
            }
        }
    }

    // 0x23 - INC HL
    fn inc_hl() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: z80::Clock {
                m: 2,
                t: 8
            },
            instruction: |cpu: &mut z80::Z80| {
                let hl = cpu.get_hl();
                let hl = cpu.inc_16(hl, false);
                cpu.set_hl(hl);
            }
        }
    }

    // 0x24 - INC H
    fn inc_h() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: z80::Clock {
                m: 1,
                t: 4
            },
            instruction: |cpu: &mut z80::Z80| {
                let h = cpu.registers.h;
                let h = cpu.inc_8(h, true);
                cpu.registers.h = h;
            }
        }
    }

    // 0x25 - DEC H
    fn dec_h() -> Opcode {
        Opcode {
            size: 1,
            clock_timing: z80::Clock {
                m: 1,
                t: 4
            },
            instruction: |cpu: &mut z80::Z80| {
                let h = cpu.registers.h;
                let h = cpu.dec_8(h, true);
                cpu.registers.h = h;
            }
        }
    }

    // 0x26 - LD H d8
    fn ld_h_d8() -> Opcode {
        Opcode {
            size: 2,
            clock_timing: z80::Clock {
                m: 2,
                t: 4
            },
            instruction: |cpu: &mut z80::Z80| {
                cpu.registers.h = cpu.mmu.read(cpu.registers.pc+1);
            }
        }
    }

    // 0x27 - DAA
    fn daa() -> Opcode {
       Opcode {
           size: 1,
           clock_timing: z80::Clock {
               m: 1,
               t: 0
           },
           instruction: |cpu: &mut z80::Z80| {
               cpu.daa();
           }
       }
    }

    // 0x28 - JR Z s8
    fn jr_z_s8() -> Opcode {
        Opcode {
            size: 0,    // Real: 2 bytes, but directly modified in instruction
            clock_timing: z80::Clock {
                m: 3,
                t: 2
            },
            instruction: |cpu: &mut z80::Z80| {
                match cpu.is_zero() {
                    true => cpu.registers.pc += 2,
                    false => {
                        let next = cpu.mmu.read(cpu.registers.pc + 1) as i8;
                        let conv = (cpu.registers.pc as u32 as i32) + (next as i32);
                        cpu.registers.pc = conv as u16;
                    },
                }
            }
        }
    }

}
