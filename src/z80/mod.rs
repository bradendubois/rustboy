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

}


impl Z80 {

    // Initialization / creation of a Z80 CPU
    #[allow(dead_code)]
    fn init() -> Z80 {

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
            clock: Clock { m: 0, t: 0 }
        }
    }


    // Basic execution of the current operation at the program-counter (PC) register
    #[allow(dead_code)]
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
    }

    /// NOP aka No-Operation
    #[allow(dead_code)]
    fn nop(&mut self) {
        self.registers.pc+=1;
    }

}