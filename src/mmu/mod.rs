pub mod byte;
mod mbc;
mod memory_map;
mod interrupts;

use interrupts::{interrupt_flag, interrupt_enable};

use mbc::{MBC, mbc0::{MBC0}, mbc1::MBC1};
use std::fmt;
use crate::cartridge::Cartridge;
use crate::ppu::PPU;
use crate::sound::Sound;
use crate::timer::Timer;
use crate::joypad::Joypad;
use crate::serial::Serial;

const W_RAM_SIZE: usize = 0x8000;
const H_RAM_SIZE: usize = 0x7F;

#[allow(unreachable_patterns)]
#[allow(dead_code)]
pub struct MMU {
    in_bios: bool,
    bios: Vec<u8>,

    w_ram: [u8; W_RAM_SIZE],
    w_ram_bank: usize,

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

            mbc: match cartridge.cartridge_type {
                0x00 ..= 0x00 => Box::new(MBC0::new(cartridge)),
                0x01 ..= 0x03 => Box::new(MBC1::new(cartridge)),
                0x05 ..= 0x06 => panic!("MBC2 not implemented!"), // MBC2::new(cartridge),
                0x0F ..= 0x13 => panic!("MBC3 not implemented!"), // MBC3::new(cartridge),
                0x19 ..= 0x1E => panic!("MBC5 not implemented!"), // MBC5::new(cartridge)

                _ => panic!("Unsupported cartridge type: {}!", cartridge.cartridge_type),
            },

            ppu: PPU::new(),
            apu: Sound::new(),
            timer: Timer::new(),
            joypad: Joypad::new(),
            serial: Serial::new(),

            interrupt_enable: 0
        };

        mmu.set_initial();

        mmu
    }

    pub fn read(&mut self, address: u16) -> u8 {

        // println!("reading: {:#06X}", address);

        match address {
            0x0000 ..= 0x3FFF => self.mbc.read_rom(address),                // ROM
            0x4000 ..= 0x7FFF => self.mbc.read_rom(address),                // Switchable ROM Bank
            0x8000 ..= 0x9FFF => self.ppu.read(address),                    // Video RAM
            0xA000 ..= 0xBFFF => self.mbc.read_ram(address),                // Switchable RAM Bank
            0xC000 ..= 0xCFFF => self.read_ram(address),                    // Internal RAM
            0xD000 ..= 0xDFFF => self.read_rambank(address),                // Internal RAM
            0xE000 ..= 0xEFFF => self.read_ram(address),                    // Internal RAM
            0xF000 ..= 0xFDFF => self.read_rambank(address),                // Echo RAM
            0xFE00 ..= 0xFE9F => self.ppu.read(address),                    // Sprite Attributes
            0xFEA0 ..= 0xFEFF => 0xFF,                                      // Unusable
            0xFF00 ..= 0xFF7F => self.read_io_registers(address),           // I/O Registers
            0xFF80 ..= 0xFFFE => self.read_hram(address),                   // High RAM
            0xFFFF ..= 0xFFFF => self.interrupt_enable,                     // Interrupt Register

            _ => panic!("Unmapped address {:#06X}", address)
        }
    }

    pub fn write(&mut self, value: u8, address: u16) {

        match address {
            0x0000 ..= 0x3FFF => self.mbc.write_rom(value, address),        // ROM
            0x4000 ..= 0x7FFF => self.mbc.write_rom(value, address),        // Switchable ROM Bank
            0x8000 ..= 0x9FFF => self.ppu.write(value, address),            // Video RAM
            0xA000 ..= 0xBFFF => self.mbc.write_ram(value, address),        // Switchable RAM Bank
            0xC000 ..= 0xCFFF => self.write_ram(value, address),            // Internal RAM
            0xD000 ..= 0xDFFF => self.write_rambank(value, address),        // Internal RAM
            0xE000 ..= 0xEFFF => self.write_ram(value, address),            // Internal RAM
            0xF000 ..= 0xFDFF => self.write_rambank(value, address),        // Echo RAM
            0xFE00 ..= 0xFE9F => self.ppu.write(value, address),            // Sprite Attributes
            0xFEA0 ..= 0xFEFF => (),                                        // Unusable
            0xFF00 ..= 0xFF7F => self.write_io_registers(value, address),   // I/O Registers
            0xFF80 ..= 0xFFFE => self.write_hram(value, address),           // High RAM
            0xFFFF ..= 0xFFFF => self.interrupt_enable = value,             // Interrupt Register

            _ => panic!("Unmapped address {:#06X}", address)
        };
    }

    pub fn read_word(&mut self, address: u16) -> u16 {
        let lower = self.read(address);
        let upper = self.read(address + 1);
        ((upper as u16) << 8) | (lower as u16)
    }

    pub fn write_word(&mut self, value: u16, address: u16) {
        let lower = (value & 0xFF) as u8;
        let upper = (value >> 8) as u8;
        self.write(lower, address);
        self.write(upper, address+1);
    }

    // PPU

    pub fn run_ppu(&mut self, cycles: u64) {
        self.ppu.run_for(cycles);
    }

    // RAM

    fn read_ram(&mut self, address: u16) -> u8 {
        self.w_ram[MMU::w_ram_conv(address)]
    }

    fn read_rambank(&mut self, address: u16) -> u8 {
        self.w_ram[self.w_rambank_conv(address)]
    }

    fn write_ram(&mut self, value: u8, address: u16) {
        self.w_ram[MMU::w_ram_conv(address)] = value;
    }

    fn write_rambank(&mut self, value: u8, address: u16) {
        self.w_ram[self.w_rambank_conv(address)] = value;
    }

    // HRAM - 0xFF80 - 0xFFFE
    fn read_hram(&mut self, address: u16) -> u8 {
        self.h_ram[(address % 0xFF80) as usize]
    }

    fn write_hram(&mut self, value: u8, address: u16) {
        self.h_ram[(address % 0xFF80) as usize] = value;
    }

    fn set_initial(&mut self) {
        self.write(0x00, 0xFF05);
        self.write(0x00, 0xFF06);
        self.write(0x00, 0xFF07);
        self.write(0x80, 0xFF10);
        self.write(0xBF, 0xFF11);
        self.write(0xF3, 0xFF12);
        self.write(0xBF, 0xFF14);
        self.write(0x3F, 0xFF16);
        self.write(0x00, 0xFF17);
        self.write(0xBF, 0xFF19);
        self.write(0x7F, 0xFF1A);
        self.write(0xFF, 0xFF1B);
        self.write(0x9F, 0xFF1C);
        self.write(0xBF, 0xFF1E);
        self.write(0xFF, 0xFF20);
        self.write(0x00, 0xFF21);
        self.write(0x00, 0xFF22);
        self.write(0xBF, 0xFF23);
        self.write(0x77, 0xFF24);
        self.write(0xF3, 0xFF25);
        self.write(0xF1, 0xFF26);
        self.write(0x91, 0xFF40);
        self.write(0x00, 0xFF42);
        self.write(0x00, 0xFF43);
        self.write(0x00, 0xFF45);
        self.write(0xFC, 0xFF47);
        self.write(0xFF, 0xFF48);
        self.write(0xFF, 0xFF49);
        self.write(0x00, 0xFF4A);
        self.write(0x00, 0xFF4B);
        self.write(0x00, 0xFFFF);
    }

    // IO Registers

    fn read_io_registers(&mut self, address: u16) -> u8 {
        match address {
            0xFF00 ..= 0xFF00=> self.joypad.read(),
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
            0xFF40 ..= 0xFF4F => self.ppu.read(address),
            0xFF50 ..= 0xFF50 => if self.in_bios { 1 } else { 0 },
            0xFF51 ..= 0xFF55 => 0,        // Color GB Only - VRAM DMA
            0xFF68 ..= 0xFF69 => 0,        // Color GB Only - BG / OBJ Palettes
            0xFF70 ..= 0xFF70 => 0,        // Color GB Only - WRAM Bank Select
            _ => panic!("Unmapped address {:#06X}", address)
        }
    }

    fn write_io_registers(&mut self, value: u8, address: u16) {
        match address {
            0xFF00 ..= 0xFF01 => self.joypad.write(value),
            0xFF01 ..= 0xFF02 => self.serial.write(value, address),
            0xFF03 ..= 0xFF03 => (),                                        // unmapped
            0xFF04 ..= 0xFF07 => self.timer.write(value, address),
            0xFF08 ..= 0xFF0E => (),                                        // unmapped
            0xFF0F ..= 0xFF0F => self.interrupt_flag_write(value),
            0xFF10 ..= 0xFF14 => self.apu.write(value, address),
            0xFF15 ..= 0xFF15 => (),                                        // unmapped
            0xFF16 ..= 0xFF1E => self.apu.write(value, address),
            0xFF1F ..= 0xFF1F => (),                                        // unmapped
            0xFF20 ..= 0xFF26 => self.apu.write(value, address),
            0xFF27 ..= 0xFF2F => (),                                        // unmapped
            0xFF30 ..= 0xFF3F => self.apu.write(value, address),
            0xFF40 ..= 0xFF4F => self.ppu.write(value, address),
            0xFF50 ..= 0xFF50 => self.in_bios = self.in_bios && value != 0,
            0xFF51 ..= 0xFF55 => (),        // Color GB Only - VRAM DMA
            0xFF68 ..= 0xFF69 => (),        // Color GB Only - BG / OBJ Palettes
            0xFF70 ..= 0xFF70 => (),        // Color GB Only - WRAM Bank Select

            _ => panic!("Unmapped address {:#06X}", address)
        }
    }

    // Interrupts

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

    // Helper

    fn w_ram_conv(address: u16) -> usize {
        address as usize & 0x0FFF
    }

    fn w_rambank_conv(&self, address: u16) -> usize {
        MMU::w_ram_conv(address) | (self.w_ram_bank * 0x1000)
    }

}


impl fmt::Debug for MMU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MMU Debug")
    }
}
