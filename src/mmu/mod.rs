use std::fmt;

pub mod byte;

mod mbc;
mod memory_map;
mod interrupts;

// Make MemoryMap public to expose to other modules
pub use super::mmu::memory_map::MemoryMap;

use interrupts::{interrupt_flag, interrupt_enable};
use mbc::{MBC, mbc0::{MBC0}, mbc1::MBC1};

use super::cartridge::Cartridge;
use super::ppu::PPU;
use super::sound::Sound;
use super::timer::Timer;
use super::joypad::Joypad;
use super::serial::Serial;

const W_RAM_SIZE: usize = 0x2000;
const H_RAM_SIZE: usize = 0x7F;

const INITIAL_MEMORY_CONTENTS: &'static [(u16, u8); 31] = &[

    // Timer
    (0xFF05, 0x00), (0xFF06, 0x00), (0xFF07, 0x00),

    // APU
    (0xFF10, 0x80), (0xFF11, 0xBF), (0xFF12, 0xF3), (0xFF14, 0xBF), (0xFF16, 0x3F), (0xFF17, 0x00),
    (0xFF19, 0xBF), (0xFF1A, 0x7F), (0xFF1B, 0xFF), (0xFF1C, 0x9F), (0xFF1E, 0xBF),

    // APU
    (0xFF20, 0xFF), (0xFF21, 0x00), (0xFF22, 0x00), (0xFF23, 0xBF), (0xFF24, 0x77), (0xFF25, 0xF3),
    (0xFF26, 0xF1),

    // PPU
    (0xFF40, 0x91), (0xFF42, 0x00), (0xFF43, 0x00), (0xFF45, 0x00), (0xFF47, 0xFC), (0xFF48, 0xFF),
    (0xFF49, 0xFF), (0xFF4A, 0x00), (0xFF4B, 0x00),

    // Interrupt Enable Flag - 0xFFFF
    (0xFFFF, 0x00)
];

#[allow(unreachable_patterns)]
#[allow(dead_code)]
pub struct MMU {
    in_bios: bool,
    bios: Vec<u8>,

    w_ram: [u8; W_RAM_SIZE],
    h_ram: [u8; H_RAM_SIZE],

    mbc: Box<dyn MBC>,
    ppu: PPU,
    apu: Sound,
    timer: Timer,
    joypad: Joypad,
    serial: Serial,

    // Corresponds to the IE (Interrupt Enable R/W) Register at 0xFFFF
    interrupt_enable: u8
}


#[allow(unreachable_patterns)]
#[allow(dead_code)]
impl MMU {

    pub fn new(cartridge: Cartridge) -> MMU {

        let mut mmu = MMU {
            in_bios: false,
            bios: vec![],

            w_ram: [0; W_RAM_SIZE],
            w_ram_bank: 1,

            h_ram: [0; H_RAM_SIZE],

            mbc: match cartridge.cartridge_type() {
                0x00 ..= 0x00 => Box::new(MBC0::new(cartridge)),
                0x01 ..= 0x03 => Box::new(MBC1::new(cartridge)),
                0x05 ..= 0x06 => panic!("MBC2 not implemented!"), // MBC2::new(cartridge),
                0x0F ..= 0x13 => panic!("MBC3 not implemented!"), // MBC3::new(cartridge),
                0x19 ..= 0x1E => panic!("MBC5 not implemented!"), // MBC5::new(cartridge)

                _ => panic!("Unsupported cartridge type: {:#4X}!", cartridge.cartridge_type()),
            },

            ppu: PPU::new(),
            apu: Sound::new(),
            timer: Timer::new(),
            joypad: Joypad::new(),
            serial: Serial::new(),

            interrupt_enable: 0
        };

        // Initialize memory contents to post-BIOS values
        for (address, value) in INITIAL_MEMORY_CONTENTS.iter() {
            mmu.write(*address, *value);
        }

        mmu
    }


    pub fn run(&mut self, cpu_cycles: u64) {
        self.ppu.run_for(cpu_cycles);
        self.timer.run((cpu_cycles / 4) as usize);
    }

    /*************************/
    /*    Read/Write Words   */
    /*************************/

    pub fn read_word(&mut self, address: u16) -> u16 {
        let lower = self.read(address);
        let upper = self.read(address + 1);
        ((upper as u16) << 8) | (lower as u16)
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        let lower = (value & 0xFF) as u8;
        let upper = (value >> 8) as u8;
        self.write(address, lower);
        self.write(address+1, upper);
    }

    /*************************/
    /*  RAM (0xC000-0xFDFF)  */
    /*************************/

    fn read_ram(&mut self, address: u16) -> u8 {
        self.w_ram[(address as usize) & (W_RAM_SIZE - 1)]
    }

    fn read_echo(&mut self, address: u16) -> u8 {
        self.read_ram(address & 0xDDFF)
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        self.w_ram[(address as usize) & (W_RAM_SIZE - 1)] = value;
    }

    fn write_echo(&mut self, address: u16, value: u8) {
        self.write_ram(address & 0xDDFF, value);
    }

    /*************************/
    /*    IO (0xFF00-FF7F)   */
    /*************************/

    fn read_io_registers(&mut self, address: u16) -> u8 {
        match address {
            0xFF00 ..= 0xFF00 => self.joypad.read(),
            0xFF01 ..= 0xFF02 => self.serial.read(address),
            0xFF03 ..= 0xFF03 => 0xFF,                              //unmapped
            0xFF04 ..= 0xFF07 => self.timer.read(address),
            0xFF08 ..= 0xFF0E => 0xFF,                              // unmapped
            0xFF0F ..= 0xFF0F => self.interrupt_flag_read(),
            0xFF10 ..= 0xFF14 => self.apu.read(address),
            0xFF15 ..= 0xFF15 => 0xFF,                              // unmapped
            0xFF16 ..= 0xFF1E => self.apu.read(address),
            0xFF1F ..= 0xFF1F => 0xFF,                              // unmapped
            0xFF20 ..= 0xFF26 => self.apu.read(address),
            0xFF27 ..= 0xFF2F => 0xFF,                              // unmapped
            0xFF30 ..= 0xFF3F => self.apu.read(address),
            0xFF40 ..= 0xFF4B => self.ppu.read(address),
            0xFF4C ..= 0xFF4C => 0xFF,                              // unmapped
            0xFF4D ..= 0xFF4D => self.ppu.read(address),
            0xFF4E ..= 0xFF4E => 0xFF,                              // unmapped
            0xFF4F ..= 0xFF4F => self.ppu.read(address),
            0xFF50 ..= 0xFF50 => if self.in_bios { 1 } else { 0 },
            0xFF51 ..= 0xFF55 => self.ppu.read(address),
            0xFF56 ..= 0xFF56 => 0xFF,    // CGB Only - RP - Infrared Comm. Port
            0xFF57 ..= 0xFF67 => 0xFF,                              // unmapped
            0xFF68 ..= 0xFF6B => self.ppu.read(address),
            0xFF6C ..= 0xFF6F => 0xFF,                              // unmapped
            0xFF70 ..= 0xFF70 => 0xFF,    // CGB Only - SVBK - WRAM Bank
            0xFF71 ..= 0xFF7F => 0xFF,                              // unmapped

            _ => panic!("unmapped io register address {:#06X}", address)
        }
    }

    fn write_io_registers(&mut self, value: u8, address: u16) {
        match address {
            0xFF00 ..= 0xFF01 => self.joypad.write(value),
            0xFF01 ..= 0xFF02 => self.serial.write(address, value),
            0xFF03 ..= 0xFF03 => (),                                        // unmapped
            0xFF04 ..= 0xFF07 => self.timer.write(address, value),
            0xFF08 ..= 0xFF0E => (),                                        // unmapped
            0xFF0F ..= 0xFF0F => self.interrupt_flag_write(value),
            0xFF10 ..= 0xFF14 => self.apu.write(value, address),
            0xFF15 ..= 0xFF15 => (),                                        // unmapped
            0xFF16 ..= 0xFF1E => self.apu.write(value, address),
            0xFF1F ..= 0xFF1F => (),                                        // unmapped
            0xFF20 ..= 0xFF26 => self.apu.write(value, address),
            0xFF27 ..= 0xFF2F => (),                                        // unmapped
            0xFF30 ..= 0xFF3F => self.apu.write(value, address),
            0xFF40 ..= 0xFF4B => self.ppu.write(address, value),
            0xFF4C ..= 0xFF4C => (),                                        // unmapped
            0xFF4D ..= 0xFF4D => self.ppu.write(address, value),
            0xFF4E ..= 0xFF4E => (),                                        // unmapped
            0xFF4F ..= 0xFF4F => self.ppu.write(address, value),
            0xFF50 ..= 0xFF50 => self.in_bios = self.in_bios && value != 0,
            0xFF51 ..= 0xFF55 => self.ppu.write(address, value),
            0xFF56 ..= 0xFF56 => (),    // CGB Only - RP - Infrared Comm. Port
            0xFF57 ..= 0xFF67 => (),                                        // unmapped
            0xFF68 ..= 0xFF6B => self.ppu.write(address, value),
            0xFF6C ..= 0xFF6F => (),                                        // unmapped
            0xFF70 ..= 0xFF70 => (),    // CGB Only - SVBK - WRAM Bank
            0xFF71 ..= 0xFF7F => (),                                        // unmapped

            _ => panic!("Unmapped address {:#06X}", address)
        }
    }

    /*************************/
    /*  HRAM (0xFF80-0xFFFE) */
    /*************************/

    fn read_hram(&mut self, address: u16) -> u8 {
        self.h_ram[(address % 0xFF80) as usize]
    }

    fn write_hram(&mut self, address: u16, value: u8) {
        self.h_ram[(address % 0xFF80) as usize] = value;
    }

    /*************************/
    /*       Interrupts      */
    /*************************/

    fn interrupt_flag_read(&self) -> u8 {

        let mut result: u8 = 0;

        if self.ppu.vblank_interrupt { result |= 0x01 };
        if self.ppu.stat_interrupt   { result |= 0x02 };
        if self.timer.interrupt      { result |= 0x04 };
        if self.serial.interrupt     { result |= 0x08 };
        if self.joypad.interrupt     { result |= 0x10 };

        result
    }

    fn interrupt_flag_write(&mut self, value: u8) {
        self.ppu.vblank_interrupt = value & 0x01 != 0;
        self.ppu.stat_interrupt   = value & 0x02 != 0;
        self.timer.interrupt      = value & 0x04 != 0;
        self.serial.interrupt     = value & 0x08 != 0;
        self.joypad.interrupt     = value & 0x10 != 0;
    }
}

impl MemoryMap for MMU {

    fn read(&mut self, address: u16) -> u8 {
        match address {
            0x0000 ..= 0x3FFF => self.mbc.read(address),                    // ROM
            0x4000 ..= 0x7FFF => self.mbc.read(address),                    // Switchable ROM Bank
            0x8000 ..= 0x9FFF => self.ppu.read(address),                    // Video RAM
            0xA000 ..= 0xBFFF => self.mbc.read(address),                    // Switchable RAM Bank
            0xC000 ..= 0xDFFF => self.read_ram(address),                    // Internal RAM
            0xE000 ..= 0xFDFF => self.read_echo(address),                   // Echo RAM
            0xFE00 ..= 0xFE9F => self.ppu.read(address),                    // Sprite Attributes
            0xFEA0 ..= 0xFEFF => 0xFF,                                      // Unusable
            0xFF00 ..= 0xFF7F => self.read_io_registers(address),           // I/O Registers
            0xFF80 ..= 0xFFFE => self.read_hram(address),                   // High RAM
            0xFFFF ..= 0xFFFF => self.interrupt_enable,                     // Interrupt Register

            _ => panic!("Unmapped address {:#06X}", address)
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000 ..= 0x3FFF => self.mbc.write(address, value),            // ROM
            0x4000 ..= 0x7FFF => self.mbc.write(address, value),            // Switchable ROM Bank
            0x8000 ..= 0x9FFF => self.ppu.write(address, value),            // Video RAM
            0xA000 ..= 0xBFFF => self.mbc.write(address, value),            // Switchable RAM Bank
            0xC000 ..= 0xDFFF => self.write_ram(address, value),            // Internal RAM
            0xE000 ..= 0xFDFF => self.write_echo(address, value),           // Echo RAM
            0xFE00 ..= 0xFE9F => self.ppu.write(address, value),            // Sprite Attributes
            0xFEA0 ..= 0xFEFF => (),                                        // Unusable
            0xFF00 ..= 0xFF7F => self.write_io_registers(value, address),   // I/O Registers
            0xFF80 ..= 0xFFFE => self.write_hram(address, value),           // High RAM
            0xFFFF ..= 0xFFFF => self.interrupt_enable = value,             // Interrupt Register

            _ => panic!("Unmapped address {:#06X}", address)
        };
    }
}

impl fmt::Debug for MMU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MMU Debug")
    }
}
