use super::mmu;
use crate::z80::Status::STOPPED;

#[derive(Debug)]
struct Clock {
    m: u16,
    t: u16
}
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
            0x00 => Opcode::nop(),
            0x01 => Opcode::ld_bc(),
            0x02 => Opcode::ld_bc_a(),
            _ => panic!("Unmapped opcode {}", code)
        }
    }

    // 0x00 - NOP ; no operation
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

    // 0x01 - LD BC, d16 ; load the 2 following bytes of immediate data into BC
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

    // 0x02 - LD (BC), A : store contents of A in memory location specified by registers BC
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


    /// 0x10 - STOP : Stops the system clock and oscillator circuit.
    /// LCD controller is also stopped.
    /// Internal RAM register ports remain unchanged
    /// Cancelled by RESET signal
    fn stop() -> Opcode {
        Opcode{
            size: 2,
            clock_timing: Clock{
                m: 1, t: 4,
            },
            instruction: |cpu: &mut Z80| {
                cpu.status = STOPPED;
            }
        }
    }

    /// 0x11 - LD DE, d16 : Loads 2 bytes of immediate data into registers D,E
    /// First byte is the lower byte, second byte is higher. Love Little endian -.-
    fn ld_de() -> Opcode {
        Opcode {
            size: 3,
            clock_timing: Clock {
                m: 3,
                t: 12,
            },
            instruction: |cpu: &mut Z80| {
                cpu.registers.e = cpu.mmu.read(cpu.registers.pc + 1);
                cpu.registers.d = cpu.mmu.read(cpu.registers.pc + 2);
            }
        }
    }

    /// 0x12 - LD (DE), A : store contents of A in memory location specified by registers DE
    fn ld_de_a()-> Opcode{
        Opcode {
            size: 1,
            clock_timing: Clock {
                m: 2,
                t: 8
            },
            instruction: |cpu: &mut Z80| {
                cpu.mmu.write(cpu.registers.a, ((cpu.registers.d << 8) + cpu.registers.e).into());
            }
        }
    }

    /// 0x13 - INC DE : Increment the contents of registers DE by 1
    fn inc_de() -> Opcode {
        Opcode{
            size:1,
            clock_timing: Clock{
                m: 2, t: 8
            },
            instruction: |cpu: &mut Z80| {
                match cpu.registers.e.checked_add(1){
                    Some(x) => cpu.registers.e += 1,
                    None => {
                        cpu.registers.e += 1;
                        match cpu.registers.e.checked_add(1){
                            Some(i) => cpu.registers.d = i,
                            None => {
                                cpu.registers.d += 1;
                                cpu.registers.f |= 0x10;
                            }
                        }
                    }
                }

            }
        }
    }

    /// 0x14 - INC D : Increment the contents of D
    fn inc_d() -> Opcode {
        Opcode{
            size: 1,
            clock_timing: Clock{m:1,t:4},
            instruction: |cpu: &mut Z80| {
                match cpu.registers.d.checked_add(1){
                    Some(x) => cpu.registers.d = x,
                    None => {cpu.registers.d += 1; cpu.registers.f |= 0x10;}
                }

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
            status: Status{RUNNING},

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
}

