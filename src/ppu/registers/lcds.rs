use crate::mmu::byte::Byte;
use crate::ppu::Mode;

// Fields taken from: http://bgb.bircd.org/pandocs.htm#lcdstatusregister
pub struct LCDS {

    // Dissected 8-bit value interpretation

    /// LYC=LY Coincidence Interrupt
    /// Bit 6 : 0/False = Disabled, 1/True = Enabled
    lyc_interrupt: bool,

    /// Mode 2 OAM Interrupt
    /// Bit 5 : 0/False = Disabled, 1/True = Enabled
    mode_2_oam_interrupt: bool,

    /// Mode 1 V-Blank Interrupt
    /// Bit 4 : 0/False = Disabled, 1/True = Enabled
    mode_1_v_blank_interrupt: bool,

    /// Mode 0  H-Blank Interrupt
    /// Bit 3 : 0/False = Disabled, 1/True = Enabled
    mode_0_h_blank_interrupt: bool,

    /// Coincidence Flag
    /// Bit 2 : 0/False = LYC != LC, 1/True = LYC == LC
    coincidence_flag: bool,

    /// Mode Flag
    /// See Mode enum.
    mode_flag: Mode,

    /// Real 8-bit value stored
    value: u8
}

#[allow(dead_code)]
impl LCDS {
    pub fn new() -> LCDS {
        LCDS {
            lyc_interrupt: false,
            mode_2_oam_interrupt: false,
            mode_1_v_blank_interrupt: false,
            mode_0_h_blank_interrupt: false,
            coincidence_flag: false,
            mode_flag: Mode::Mode0,
            value: 0
        }
    }

    /// LYC = LY STATE Interrupt Source
    pub fn lyc_interrupt(&self) -> bool { self.lyc_interrupt }

    /// Mode 2 OAM STAT Interrupt Source
    pub fn mode_2_oam_interrupt(&self) -> bool { self.mode_2_oam_interrupt }

    /// Mode 1 VBlank STAT Interrupt source
    pub fn mode_1_v_blank_interrupt(&self) -> bool { self.mode_1_v_blank_interrupt }

    /// Mode 0 HBlank STAT Interrupt source
    pub fn mode_0_h_blank_interrupt(&self) -> bool { self.mode_0_h_blank_interrupt }

    /// LYC = LY Flag
    pub fn coincidence_flag(&self) -> bool { self.coincidence_flag }

    /// Mode Flag
    pub fn mode_flag(&self) -> Mode { self.mode_flag }
}

impl Byte for LCDS {

    fn read(&self) -> u8 {
        self.value
    }

    /// Write given value into LCDS Register, populating ease-of-access fields
    fn write(&mut self, value: u8) {

        self.value = value;

        self.lyc_interrupt = value & 0x40 != 0;                 // Bit 6
        self.mode_2_oam_interrupt = value & 0x20 != 0;          // Bit 5
        self.mode_1_v_blank_interrupt = value & 0x10 != 0;      // Bit 4
        self.mode_0_h_blank_interrupt = value & 0x08 != 0;      // Bit 3
        self.coincidence_flag = value & 0x04 != 0;              // Bit 2

        // Bits 0-1
        self.mode_flag = match value & 0x03 {
            0b00 => Mode::Mode0,
            0b01 => Mode::Mode1,
            0b10 => Mode::Mode2,
            0b11 => Mode::Mode3,

            _ => panic!("arithmetic error: {}", value)
        };
    }
}