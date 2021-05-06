use super::mmu;

#[derive(Debug)]
struct Clock {
    m: u16,
    t: u16
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
    m: u16,
    t: u16,
}

// Suggestion: Maybe this can come in handy
#[allow(dead_code)]
type Opcode = u8;


// Struct representing the Z80 CPU
#[derive(Debug)]
struct Z80 {

    // Struct of all registers in the Z80
    registers: Registers,

    // Struct representing the clock of the Z80 for purposes of timing
    clock: Clock,

    // Struct representing the memory unit
    mmu: mmu::MMU
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
                m: 0,
                t: 0
            },

            // Clock begins at 0
            clock: Clock { m: 0, t: 0 },

            // MMU Unit
            mmu
        }
    }


    // Basic execution of the current operation at the program-counter (PC) register
    fn step(&mut self) {

        // Test variable to test the match statement
        let x = 3;

        /* Basic match of opcode -> function execution, catch-all at end will be to
         * panic on any unmapped opcode execution as this should (probably) never occur
         * */
        match x {
            0x00 => self.nop(),
            _ => panic!("execution of unmapped opcode: {}", x)
        };

        // Increment program counter to next position in memory; it's unsigned, 16 bit, so overflow
        // may occur and resets back to position 0
        self.registers.pc += 1;
    }


    // 0x00 - NOP ; no operation
    fn nop (&mut self) { }

    // 0x01 - LD BC, d16 ; load the 2 following bytes of immediate data into BC
    fn ld_bc(&mut self) {
        self.registers.c = self.mmu.read(self.registers.pc + 1);
        self.registers.b = self.mmu.read(self.registers.pc + 2);
    }

    // 0x02 - LD (BC), A : store contents of A in memory location specified by registers BC
    fn ld_bc_a(&mut self) {
        self.mmu.write(self.registers.a, ((self.registers.b << 8) + self.registers.c).into());
    }
}
