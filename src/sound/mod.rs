use crate::traits::{MemoryMap, RunComponent};

#[allow(dead_code)]
pub struct Sound {

    // Channel 1 : Tone & Sweep - 0xFF10-0xFF14
    channel_1_sweep: u8,                // 0xFF10
    channel_1_length_pattern: u8,       // 0xFF11
    channel_1_volume_envelope: u8,      // 0xFF12
    channel_1_frequency_lo: u8,         // 0xFF13
    channel_1_frequency_hi: u8,         // 0xFF14

    // Channel 2 : Tone - 0xFF16-0xFF19
    channel_2_length_pattern: u8,       // 0xFF16
    channel_2_volume_envelope: u8,      // 0xFF17
    channel_2_frequency_lo: u8,         // 0xFF18
    channel_2_frequency_hi: u8,         // 0xFF19

    // Channel 3 : Wave Output - 0xFF1A-0xFF1E
    channel_3_sound_enable: u8,         // 0xFF1A
    channel_3_sound_length: u8,         // 0xFF1B
    channel_3_select_output_level: u8,  // 0xFF1C
    channel_3_frequency_lo: u8,         // 0xFF1D
    channel_3_frequency_hi: u8,         // 0xFF1E

    // Wave Pattern RAM
    wave_pattern_data: [u8; 16],        // 0xFF30 - FF3F

    // Channel 4 : Noise - 0xFF20-0xFF23
    channel_4_sound_length: u8,         // 0xFF20
    channel_4_volume_envelope: u8,      // 0xFF21
    channel_4_polynomial_counter: u8,   // 0xFF22
    channel_4_counter_consecutive: u8,  // 0xFF23

    // Sound Control Registers - 0xFF24-0xFF26
    channel_control_on_off: u8,         // 0xFF24
    sound_output_terminal: u8,          // 0xFF25
    sound_on_off: u8,                   // 0xFF26
}


impl Sound {

    pub fn new() -> Sound {
        Sound {
            channel_1_sweep: 0,
            channel_1_length_pattern: 0,
            channel_1_volume_envelope: 0,
            channel_1_frequency_lo: 0,
            channel_1_frequency_hi: 0,

            channel_2_length_pattern: 0,
            channel_2_volume_envelope: 0,
            channel_2_frequency_lo: 0,
            channel_2_frequency_hi: 0,

            channel_3_sound_enable: 0,
            channel_3_sound_length: 0,
            channel_3_select_output_level: 0,
            channel_3_frequency_lo: 0,
            channel_3_frequency_hi: 0,
            wave_pattern_data: [0; 16],

            channel_4_sound_length: 0,
            channel_4_volume_envelope: 0,
            channel_4_polynomial_counter: 0,
            channel_4_counter_consecutive: 0,

            channel_control_on_off: 0,
            sound_output_terminal: 0,
            sound_on_off: 0
        }
    }
}


impl MemoryMap for Sound {

    fn read(&mut self, address: u16) -> u8 {
        match address {
            // Channel 1 : Tone & Sweep - 0xFF10-0xFF14
            0xFF10 => self.channel_1_sweep,
            0xFF11 => self.channel_1_length_pattern,
            0xFF12 => self.channel_1_volume_envelope,
            0xFF13 => self.channel_1_frequency_lo,
            0xFF14 => self.channel_1_frequency_hi,

            // Channel 2 : Tone - 0xFF16-0xFF19
            0xFF16 => self.channel_2_length_pattern,
            0xFF17 => self.channel_2_volume_envelope,
            0xFF18 => self.channel_2_frequency_lo,
            0xFF19 => self.channel_2_frequency_hi,

            // Channel 3 : Wave Output - 0xFF1A-0xFF1E
            0xFF1A => self.channel_3_sound_enable,
            0xFF1B => self.channel_3_sound_length,
            0xFF1C => self.channel_3_select_output_level,
            0xFF1D => self.channel_3_frequency_lo,
            0xFF1E => self.channel_3_frequency_hi,

            // Channel 4 : Noise - 0xFF20-0xFF23
            0xFF20 => self.channel_4_sound_length,
            0xFF21 => self.channel_4_volume_envelope,
            0xFF22 => self.channel_4_polynomial_counter,
            0xFF23 => self.channel_4_counter_consecutive,

            // Sound Control Registers - 0xFF24-0xFF26
            0xFF24 => self.channel_control_on_off,
            0xFF25 => self.sound_output_terminal,
            0xFF26 => self.sound_on_off,

            // Wave Pattern RAM - 0xFF30 - FF3F
            0xFF30..=0xFF3F => self.wave_pattern_data[(address % 0xFF30) as usize],

            _ => panic!("unmapped sound register: {:060X}", address)
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            // Channel 1 : Tone & Sweep - 0xFF10-0xFF14
            0xFF10 => self.channel_1_sweep = value,
            0xFF11 => self.channel_1_length_pattern = value,
            0xFF12 => self.channel_1_volume_envelope = value,
            0xFF13 => self.channel_1_frequency_lo = value,
            0xFF14 => self.channel_1_frequency_hi = value,

            // Channel 2 : Tone - 0xFF16-0xFF19
            0xFF16 => self.channel_2_length_pattern = value,
            0xFF17 => self.channel_2_volume_envelope = value,
            0xFF18 => self.channel_2_frequency_lo = value,
            0xFF19 => self.channel_2_frequency_hi = value,

            // Channel 3 : Wave Output - 0xFF1A-0xFF1E
            0xFF1A => self.channel_3_sound_enable = value,
            0xFF1B => self.channel_3_sound_length = value,
            0xFF1C => self.channel_3_select_output_level = value,
            0xFF1D => self.channel_3_frequency_lo = value,
            0xFF1E => self.channel_3_frequency_hi = value,

            // Channel 4 : Noise - 0xFF20-0xFF23
            0xFF20 => self.channel_4_sound_length = value,
            0xFF21 => self.channel_4_volume_envelope = value,
            0xFF22 => self.channel_4_polynomial_counter = value,
            0xFF23 => self.channel_4_counter_consecutive = value,

            // Sound Control Registers - 0xFF24-0xFF26
            0xFF24 => self.channel_control_on_off = value,
            0xFF25 => self.sound_output_terminal = value,
            0xFF26 => self.sound_on_off = value,

            // Wave Pattern RAM - 0xFF30 - FF3F
            0xFF30..=0xFF3F => self.wave_pattern_data[(address % 0xFF30) as usize] = value,

            _ => panic!("unmapped sound register: {:060X}", address)
        };
    }
}


impl RunComponent for Sound {

    fn run(&mut self, _cpu_clock_cycles: u64) {
        // TODO
    }
}