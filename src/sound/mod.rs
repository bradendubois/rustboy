use crate::traits::MemoryMap;

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

    fn read(&self, address: u16) -> u8 {
        let _address = address;
        0
    }

    fn write(&mut self, value: u8, address: u16) {
        let _value = value;
        let _address = address;
    }
}