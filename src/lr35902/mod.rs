mod instructions;
mod status;
mod registers;
mod ime;

use std::fmt;

use status::Status;
use registers::Registers;

use super::mmu::MMU;
use crate::cartridge::Cartridge;
use crate::lr35902::ime::IME;
use std::process::exit;
use crate::lr35902::status::Status::{HALTED, RUNNING};
use std::ptr::eq;

// Struct representing the LR35902 CPU
pub struct LR35902 {

    // Struct representing the memory unit
    pub mmu: MMU,

    // Struct of all registers in the LR35902
    pub registers: Registers,

    // Enum representing the LR35902's current running status
    pub status: Status,

    // IME - Interrupt Master Enable Flag
    ime: IME,

    // Struct representing the clock of the LR35902 for purposes of timing
    pub clock: u64,

    // CB Flag : Will set whether to use the default table or the CB Prefix table
    pub use_cb_table: bool,

    mooneye_testing: bool
}

#[allow(dead_code)]
impl LR35902 {

    /// Initializer for a LR35902 CPU
    pub fn new(cartridge: Cartridge) -> LR35902 {

        LR35902 {
            mmu: MMU::new(cartridge),
            registers: Registers::new(),
            status: Status::RUNNING,
            ime: IME::Disabled,
            clock: 0,
            use_cb_table: false,
            mooneye_testing: false
        }
    }

    /// Run the cycle until otherwise halted / interrupted by an interrupt / exception
    pub fn run(&mut self) {

        // println!("cpu beginning run");

        loop {
            self.step();

            let cycles = self.step();
            if cycles == 0 {
                break;
            }

            // println!("cycles taken: {}", cycles);

            // Adjust clock and program counter (PC)
            self.clock += cycles as u64;

            // The PPU runs at a clock rate of 4.2 MHz, while the LR35902 runs at 1.05 MHz
            //  Each cycle run by the CPU corresponds to 4 PPU cycles
            self.mmu.run(cycles * 4);
        }

        // println!("cpu halted with status: {:?}", self.status);
    }


    /// Run one step the CPU, fetching/decoding/executing at the PC
    pub fn step(&mut self) -> u64 {

        // The IME has a delay of one cycle, so when 're-enabled' there must be a delay
        //  before actually re-enabling it.
        self.ime = match self.ime {
            IME::OneCycleDelay => IME::ReadyToEnable,
            IME::ReadyToEnable => IME::Enabled,

            // No Change
            IME::Enabled => IME::Enabled,
            IME::Disabled => IME::Disabled
        };

        // Handle any interrupts
        if self.ime == IME::Enabled {

            let ff0f = self.mmu.read(0xFF0F);
            let ffff = self.mmu.read(0xFFFF);

            let interrupts = ff0f & ffff;
            if interrupts != 0 {
                let handle = interrupts.trailing_zeros() as u8;
                self.mmu.write(ff0f & !(1<< handle), 0xFF0F);

                let interrupt_vector: u16 = match handle {
                    0 => 0x0040,
                    1 => 0x0048,
                    2 => 0x0050,
                    3 => 0x0058,
                    4 => 0x0060,

                    _ => panic!("invalid interrupt bit: {}", handle)
                };

                self.call(interrupt_vector);
                return 4;
            }
        } else if self.status == HALTED {
            self.status = RUNNING;
            return self.nop_0x00();
        }

        // Interrupts disabled, or none to handle
        //println!("program counter: {:#06X}", self.registers.pc);

        // Get the opcode number to execute
        let opcode = self.byte();

        //println!("fetched instruction: {:#02X}", opcode);

        if self.mooneye_testing && opcode == 0x40 {
            self.status = Status::HALTED;
            return 0;
        }

        // Execute from standard table
        self.call_instruction(opcode)
    }

    /*************************/
    /*  Program Counter (PC) */
    /*************************/

    pub fn byte(&mut self) -> u8 {
        let next_byte = self.mmu.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        next_byte
    }

    pub fn word(&mut self) -> u16 {
        let lower = self.byte();
        let upper = self.byte();
        LR35902::u16_from_u8(upper, lower)
    }

    /*************************/
    /*   Stack Pointer (SP)  */
    /*************************/

    /// Push 16 bits to the stack (SP)
    pub fn push_sp(&mut self, value: u16) {
        self.registers.sp = self.registers.sp.wrapping_sub(2);
        self.mmu.write_word(value, self.registers.sp);
    }

    /// Pop and return 16 bits from the stack (SP)
    pub fn pop_sp(&mut self) -> u16 {
        let value = self.mmu.read_word(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(2);
        value
    }

    /*************************/
    /*        Control        */
    /*************************/

    /// RST - Pseudo-instruction to call a specific address depending on opcode
    pub fn rst(&mut self, rst: u16) {
        self.call(rst);
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
    /*          IME          */
    /*************************/

    /// Get the Interrupt Master Enable flag
    pub fn is_ime(&self) -> bool {
        self.ime == IME::Enabled
    }

    /// Set the Interrupt Master Enable flag
    pub fn set_ime(&mut self) {
        self.ime = IME::OneCycleDelay;
    }

    /// Unset the IME flag
    pub fn unset_ime(&mut self) {
        self.ime = IME::Disabled;
    }

    /*************************/
    /*         STATUS        */
    /*************************/

    pub fn stop(&mut self) {
        self.mmu.write(0, 0xFF04);  // A STOP clears the timer
        self.status = Status::STOPPED;
    }

    pub fn halt(&mut self) {
        self.status = Status::HALTED;
    }

    /*************************/
    /*          ALU          */
    /*************************/

    /***      Addition     ***/

    /// Add two u8s together, handling overflow and the Z/N/H/C flags of the F register
    fn alu_add_8(&mut self, s: u8, t: u8) -> u8 {

        let result = s.wrapping_add(t);

        match result == 0 {
            true  => self.registers.set_zero(),
            false => self.registers.unset_zero(),
        };

        self.registers.unset_subtraction();

        match (s & 0xF) + (t & 0xF) > 0xF {
            true  => self.registers.set_half_carry(),
            false => self.registers.unset_full_carry(),
        };

        match s < t {
            true  => self.registers.set_full_carry(),
            false => self.registers.unset_full_carry(),
        };

        result
    }

    /// Add two u16s together, handling overflow and the Z/N/H/C flags of the F register
    pub fn add_16(&mut self, s: u16, t: u16) -> u16 {

        let result = s.wrapping_add(t);

        match (s & 0x07FF) + (t & 0x07FF) > 0x07FF {
            true  => self.registers.set_half_carry(),
            false => self.registers.unset_full_carry(),
        };

        self.registers.unset_subtraction();

        match s.checked_add(t) {
            None    => self.registers.set_full_carry(),
            Some(_) => self.registers.unset_full_carry(),
        };

        result
    }

    pub fn add_16_immediate(&mut self, s: u16, t: u8) -> u16 {

        let i = t as i8 as i16 as u16;

        self.registers.unset_subtraction();
        self.registers.unset_zero();

        match (s & 0x000F) + (i & 0x000F) > 0x000F {
            true  => self.registers.set_half_carry(),
            false => self.registers.unset_full_carry()
        };

        match (s & 0x00FF) + (i & 0x00FF) > 0x00FF {
            true  => self.registers.set_full_carry(),
            false => self.registers.unset_full_carry()
        };

        s.wrapping_add(i)
    }

    pub fn hl_add_16(&mut self, value: u16) {
        let result = self.add_16(self.registers.get_hl(), value);
        self.registers.set_hl(result);
    }

    pub fn a_add_8(&mut self, value: u8) {
        self.registers.a = self.alu_add_8(self.registers.a, value);
    }

    /// ADC - Add the given value and the carry (C) flag to the accumulator (A) register
    pub fn adc_8(&mut self, s: u8) {
        self.a_add_8(s + if self.registers.is_full_carry() { 1 } else { 0 });
    }

    /***    Subtraction    ***/

    /// Subtract t (u8) from s (u8), handling underflow and the Z/N/H/C flags of the F register
    pub fn a_sub_8(&mut self, value: u8) -> u8 {

        let result = self.registers.a.wrapping_sub(value);

        match result == 0 {
            true  => self.registers.set_zero(),
            false => self.registers.unset_zero(),
        };

        self.registers.set_subtraction();

        match (self.registers.a & 0x0F) < (value & 0x0F) {
            true  => self.registers.set_half_carry(),
            false => self.registers.unset_full_carry(),
        };

        match (self.registers.a as u16) < (value as u16) {
            true  => self.registers.set_full_carry(),
            false => self.registers.unset_full_carry()
        } ;

        self.registers.a = result;

        result
    }

    /// SBC - Subtract given value and carry flag from the A register
    pub fn sbc_8(&mut self, s: u8) {
        self.a_sub_8(s + if self.registers.is_full_carry() { 1 } else { 0 });
    }

    /***   Incrementation  ***/

    /// Increment a given u8, handling overflow and the Z/N/H/C flags of the F register
    pub fn inc_8(&mut self, s: u8) -> u8 {

        let result = s.wrapping_add(1);

        match result == 0 {
            true  => self.registers.set_zero(),
            false => self.registers.unset_zero()
        };

        match (result & 0x0F) + 1 > 0x0F {
            true  => self.registers.set_half_carry(),
            false => self.registers.unset_full_carry()
        };

        self.registers.unset_subtraction();

        result
    }

    /***    Decrementing   ***/

    /// Decrement a given u8, handling overflow and the Z/N/H/C flags of the F register
    pub fn dec_8(&mut self, s: u8) -> u8 {

        let result = s.wrapping_sub(1);

        match result == 0 {
            true  => self.registers.set_zero(),
            false => self.registers.unset_zero()
        };

        match (s & 0x0F) == 0 {
            true  => self.registers.set_half_carry(),
            false => self.registers.unset_full_carry()
        };

        self.registers.set_subtraction();

        result
    }

    /*************************/
    /*   Bitwise Operations  */
    /*************************/

    /// AND - AND the given value with the accumulator register (A) and store the result in A
    pub fn and(&mut self, t: u8) {
        self.registers.a &= t;

        match self.registers.a == 0 {
            true  => self.registers.set_zero(),
            false => self.registers.unset_zero(),
        };

        self.registers.set_half_carry();
        self.registers.unset_full_carry();
        self.registers.unset_subtraction();
    }

    /// OR - OR the given value with register A. Store result in A.
    pub fn or(&mut self, t: u8) {
        self.registers.a |= t;

        match self.registers.a == 0 {
            true  => self.registers.set_zero(),
            false => self.registers.unset_zero(),
        };
        self.registers.unset_subtraction();
        self.registers.unset_full_carry();
        self.registers.unset_full_carry();
    }

    /// CP - Compare the given value with register A, setting the zero flag if they're equal
    pub fn cp(&mut self, value: u8) {
        let restore = self.registers.a;
        if self.a_sub_8(value) == 0 {
            self.registers.set_zero()
        }
        self.registers.a = restore;
    }

    /// XOR - XOR the given value with the accumulator register (A) and store the result in A
    pub fn xor(&mut self, v: u8) {
        self.registers.a ^= v;

        match self.registers.a == 0 {
            true  => self.registers.set_zero(),
            false => self.registers.unset_zero(),
        };

        self.registers.unset_subtraction();
        self.registers.unset_full_carry();
        self.registers.unset_full_carry();
    }

    /// SWAP - return the value with higher order bits swapped with lower order bits
    pub fn swap(&mut self, s: u8) -> u8 {

        let result = (s << 4) | (s >> 4);
        match result == 0 {
            true => self.registers.set_zero(),
            false => self.registers.unset_zero(),
        };

        self.registers.unset_subtraction();
        self.registers.unset_full_carry();
        self.registers.unset_full_carry();
        result
    }

    /// BIT - Store the complement of bit b of s in the Zero (Z) flag
    pub fn bit(&mut self, s: u8, b: u8) {
        match (s & (1 << b)) == 0 {
            true  => self.registers.set_zero(),
            false => self.registers.unset_zero(),
        };

        self.registers.unset_subtraction();
        self.registers.set_half_carry();
    }

    /*************************/
    /*         ROTATE        */
    /*************************/

    /// RRC - Rotate a number right, and copy the right-most bit shifted into the C register
    pub fn rrc(&mut self, v: u8) -> u8 {

        let result = (v >> 1) | (v << 7);

        match result == 0 {
            true  => self.registers.set_zero(),
            false => self.registers.unset_zero(),
        };

        self.registers.unset_subtraction();
        self.registers.unset_full_carry();

        match result & 0x80 != 0 {
            true  => self.registers.set_full_carry(),
            false => self.registers.unset_full_carry(),
        };

        result
    }

    /// RR - Rotate a number right, copy carry flag into right-most bit
    pub fn rr(&mut self, v: u8) -> u8 {

        let carry_bit = if self.registers.is_full_carry() { 1 } else { 0 };
        let result = v >> 7 | carry_bit;

        match result == 0 {
            true  => self.registers.set_zero(),
            false => self.registers.unset_zero(),
        };

        match result & 0x80 != 0 {
            true  => self.registers.set_full_carry(),
            false => self.registers.unset_full_carry(),
        };

        self.registers.unset_subtraction();
        self.registers.unset_full_carry();

        result
    }

    /// RLC - Rotate a number left, and copy the left-most bit shifted into the C register
    pub fn rlc(&mut self, v: u8) -> u8 {
        let result = (v << 1) | (v >> 7);

        match result == 0 {
            true  => self.registers.set_zero(),
            false => self.registers.unset_zero(),
        };

        self.registers.unset_subtraction();
        self.registers.unset_full_carry();

        match result & 0x01 != 0 {
            true  => self.registers.set_full_carry(),
            false => self.registers.unset_full_carry(),
        };

        result
    }

    /// RL - Rotate a number left, copy the contents of carry into the result
    pub fn rl(&mut self, v: u8) -> u8 {
        let carry_bit = if self.registers.is_full_carry() { 1 } else { 0 };
        let result = v << 1 | carry_bit;

        match result == 0 {
            true  => self.registers.set_zero(),
            false => self.registers.unset_zero(),
        };

        match result & 0x01 != 0 {
            true  => self.registers.set_full_carry(),
            false => self.registers.unset_full_carry(),
        };

        self.registers.unset_subtraction();
        self.registers.unset_full_carry();

        result
    }

    /*************************/
    /*         SHIFT         */
    /*************************/

    /// SLA - Shift a number left, and copy the left-most bit shifted into the C register
    pub fn sla(&mut self, v: u8) -> u8 {
        let result = v << 1;

        match result == 0 {
            true  => self.registers.set_zero(),
            false => self.registers.unset_zero(),
        };

        self.registers.unset_subtraction();
        self.registers.unset_full_carry();

        match v & 0x80 != 0 {
            true  => self.registers.set_full_carry(),
            false => self.registers.unset_full_carry(),
        };

        result
    }

    /// SRA - Shift a number right, and copy the right-most bit shifted into the C register
    pub fn sra(&mut self, v: u8) -> u8 {
        let result = v >> 1;

        match result == 0 {
            true  => self.registers.set_zero(),
            false => self.registers.unset_zero(),
        };

        self.registers.unset_subtraction();
        self.registers.unset_full_carry();

        match v & 0x01 != 0 {
            true  => self.registers.set_full_carry(),
            false => self.registers.unset_full_carry(),
        };

        result
    }

    /// SRL - shift number right, copy bit 0 to CY and set bit 7 of number to 0
    pub fn srl(&mut self, r: u8) -> u8 {

        let result = r >> 1;

        match result == 0 {
            true  => self.registers.set_zero(),
            false => self.registers.unset_zero(),
        };

        match r & 0x01 != 0 {
            true  => self.registers.set_full_carry(),
            false => self.registers.unset_full_carry(),
        };

        self.registers.unset_subtraction();
        self.registers.unset_full_carry();

        result
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
    /*    Mooneye Testing    */
    /*************************/

    /// Helper method to test with a Mooneye ROM
    pub fn mooneye(path: &String) -> bool {

        println!("testing path: {}", path);

        let data = std::fs::read(path).unwrap();
        let cartridge = Cartridge::new(data);

        let mut cpu = LR35902 {
            mmu: MMU::new(cartridge),
            registers: Registers::new(),
            status: Status::RUNNING,
            ime: IME::Enabled,
            clock: 0,
            use_cb_table: false,
            mooneye_testing: true
        };

        cpu.run();

        // Test success results in fibonacci sequence in registers
        return
            cpu.registers.b ==  3 &&
            cpu.registers.c ==  5 &&
            cpu.registers.d ==  8 &&
            cpu.registers.e == 13 &&
            cpu.registers.h == 21 &&
            cpu.registers.l == 34
    }
}

impl fmt::Debug for LR35902 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\
            Registers\n\
            ========\n\
            {:?}\n
            ========\n\
            Clock: {:?}\n\
            CB Prefix Set: {:?}", self.registers, self.clock, self.use_cb_table)

    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::path::Path;

    // Root
    const MOONEYE: &str = "./roms/testing/mooneye";

    fn full_path(subdirectory: &str) -> String {
        format!("{}/{}", MOONEYE, subdirectory)
    }

    fn mooneye_all(dir: &str) {

        let mut successful: Vec<String> = Vec::new();
        let mut errors: Vec<String> = Vec::new();

        let full_dir = &format!("{}/{}", MOONEYE, dir);

        for entry in std::fs::read_dir(full_dir).unwrap() {

            let path = entry.unwrap().path();
            let pathname = path.to_str().unwrap().to_string();

            if path.is_file() && pathname.ends_with(".gb") {
                match LR35902::mooneye(&pathname) {
                    true  => successful.push(pathname),
                    false => errors.push(pathname)
                }
            }
        }

        if errors.len() == 0 {
            println!("{} : {} tests : all successful", dir, successful.len());
        } else {
            println!("{} - successful", dir);
            for success in successful.iter() {
                println!("  {}", success);
            }

            println!("{} - errors", dir);
            for error in errors.iter() {
                println!("  {}", error);
            }

            panic!("errors in {}", dir)
        }
    }

    #[test]
    fn acceptance_root() {
        mooneye_all("acceptance");
    }

    #[test]
    fn acceptance_bits() {
        mooneye_all("acceptance/bits");
    }

    #[test]
    fn acceptance_instr() {
        mooneye_all("acceptance/instr");
    }

    #[test]
    fn acceptance_interrupts() {
        mooneye_all("acceptance/interrupts");
    }
    /*

    #[test]
    fn acceptance_oam_dma() {
        mooneye_all(&format!("{}/{}", MOONEYE, "acceptance/oam_dma"));
    }
     */

    #[test]
    fn acceptance_ppu() {
        mooneye_all("acceptance/ppu");
    }
    /*

    #[test]
    fn acceptance_serial() {
        mooneye_all(&format!("{}/{}", MOONEYE, "acceptance/serial"));
    }


     */

    #[test]
    fn acceptance_timer() {
        mooneye_all("acceptance/timer");
    }


    #[test]
    fn acceptance_mbc1() {
        mooneye_all("emulator-only/mbc1");
    }

}