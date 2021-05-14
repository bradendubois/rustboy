use super::z80::Status;
use super::z80::Z80;

impl Z80 {
    /// Call the instruction corresponding the given opcode, and return the number of cycles taken
    pub fn call_instruction(&mut self, code: u8) -> u64 {
        match self.use_cb_table {
            // Default LR35902 Opcodes
            false => {
                match code {
                    // 0x0X
                    0x00 => self.nop_0x00(),
                    0x01 => self.ld_bc_0x01(),
                    0x02 => self.ld_bc_a_0x02(),
                    0x03 => self.inc_bc_0x03(),
                    0x04 => self.inc_b_0x04(),
                    0x05 => self.dec_b_0x05(),
                    0x06 => self.ld_b_0x06(),
                    0x07 => self.rlca_0x07(),
                    0x08 => self.ld_a16_sp_0x08(),
                    0x09 => self.add_hl_bc_0x09(),
                    0x0A => self.ld_a_bc_0x0a(),
                    0x0B => self.dec_bc_0x0b(),
                    0x0C => self.inc_c_0x0c(),
                    0x0D => self.dec_c_0x0d(),
                    0x0E => self.ld_c_0x0e(),
                    0x0F => self.rrca_0x0f(),

                    // 0x1x
                    0x10 => self.stop_0x10(),
                    0x11 => self.ld_de_0x11(),
                    0x12 => self.ld_de_a_0x12(),
                    0x13 => self.inc_de_0x13(),
                    0x14 => self.inc_d_0x14(),
                    0x15 => self.dec_d_0x15(),
                    0x16 => self.ld_d_0x16(),
                    0x17 => self.rla_0x17(),
                    0x18 => self.jr_s8_0x18(),
                    0x19 => self.add_hl_de_0x19(),
                    0x1A => self.ld_a_de_0x1a(),
                    0x1B => self.dec_de_0x1b(),
                    0x1C => self.inc_e_0x1c(),
                    0x1D => self.dec_e_0x1d(),
                    0x1E => self.ld_e_d8_0x1e(),
                    0x1F => self.rra_0x1f(),

                    // 0x2X
                    0x20 => self.jr_nz_s8_0x20(),
                    0x21 => self.ld_hl_d16_0x21(),
                    0x22 => self.ld_hlp_a_0x22(),
                    0x23 => self.inc_hl_0x23(),
                    0x24 => self.inc_h_0x24(),
                    0x25 => self.dec_h_0x25(),
                    0x26 => self.ld_h_d8_0x26(),
                    0x27 => self.daa_0x27(),
                    0x28 => self.jr_z_s8_0x28(),
                    0x29 => self.add_hl_hl_0x29(),
                    0x2A => self.ld_a_hlp_0x2a(),
                    0x2B => self.dec_hl_0x2b(),
                    0x2C => self.inc_l_0x2c(),
                    0x2D => self.dec_l_0x2d(),
                    0x2E => self.ld_l_d8_0x2e(),
                    0x2F => self.cpl_0x2f(),

                    //0x3x
                    0x30 => self.jr_nc_s8_0x30(),
                    0x31 => self.ld_sp_d16_0x31(),
                    0x32 => self.ld_hls_a_0x32(),
                    0x33 => self.inc_sp_0x33(),
                    0x34 => self.inc_hl_0x34(),
                    0x35 => self.dec_hl_0x35(),
                    0x36 => self.ld_hl_d8_0x36(),
                    0x37 => self.scf_0x37(),
                    0x38 => self.jr_c_s8_0x38(),
                    0x39 => self.add_hl_sp_0x39(),
                    0x3A => self.ld_a_hls_0x3a(),
                    0x3B => self.dec_sp_0x3b(),
                    0x3C => self.inc_a_0x3c(),
                    0x3D => self.dec_a_0x3d(),
                    0x3E => self.ld_a_d8_0x3e(),
                    0x3F => self.ccf_0x3f(),

                    // 0x4X
                    0x40 => self.ld_b_b_0x40(),
                    0x41 => self.ld_b_c_0x41(),
                    0x42 => self.ld_b_d_0x42(),
                    0x43 => self.ld_b_e_0x43(),
                    0x44 => self.ld_b_h_0x44(),
                    0x45 => self.ld_b_l_0x45(),
                    0x46 => self.ld_b_hl_0x46(),
                    0x47 => self.ld_b_a_0x47(),
                    0x48 => self.ld_c_b_0x48(),
                    0x49 => self.ld_c_c_0x49(),
                    0x4A => self.ld_c_d_0x4a(),
                    0x4B => self.ld_c_e_0x4b(),
                    0x4C => self.ld_c_h_0x4c(),
                    0x4D => self.ld_c_l_0x4d(),
                    0x4E => self.ld_c_hl_0x4e(),
                    0x4F => self.ld_c_a_0x4f(),

                    // 0x5X
                    0x50 => self.ld_d_b_0x50(),
                    0x51 => self.ld_d_c_0x51(),
                    0x52 => self.ld_d_d_0x52(),
                    0x53 => self.ld_d_e_0x53(),
                    0x54 => self.ld_d_h_0x54(),
                    0x55 => self.ld_d_l_0x55(),
                    0x56 => self.ld_d_hl_0x56(),
                    0x57 => self.ld_d_a_0x57(),
                    0x58 => self.ld_e_b_0x58(),
                    0x59 => self.ld_e_c_0x59(),
                    0x5A => self.ld_e_d_0x5a(),
                    0x5B => self.ld_e_e_0x5b(),
                    0x5C => self.ld_e_h_0x5c(),
                    0x5D => self.ld_e_l_0x5d(),
                    0x5E => self.ld_e_hl_0x5e(),
                    0x5F => self.ld_e_a_0x5f(),

                    // 0x6X
                    0x60 => self.ld_h_b_0x60(),
                    0x61 => self.ld_h_c_0x61(),
                    0x62 => self.ld_h_d_0x62(),
                    0x63 => self.ld_h_e_0x63(),
                    0x64 => self.ld_h_h_0x64(),
                    0x65 => self.ld_h_l_0x65(),
                    0x66 => self.ld_h_hl_0x66(),
                    0x67 => self.ld_h_a_0x67(),
                    0x68 => self.ld_l_b_0x68(),
                    0x69 => self.ld_l_c_0x69(),
                    0x6A => self.ld_l_d_0x6a(),
                    0x6B => self.ld_l_e_0x6b(),
                    0x6C => self.ld_l_h_0x6c(),
                    0x6D => self.ld_l_l_0x6d(),
                    0x6E => self.ld_l_hl_0x6e(),
                    0x6F => self.ld_l_a_0x6f(),

                    // 0x7X


                    0x70 => self.ld_hl_b_0x70(),
                    0x71 => self.ld_hl_c_0x71(),
                    0x72 => self.ld_hl_d_0x72(),
                    0x73 => self.ld_hl_e_0x73(),
                    0x74 => self.ld_hl_h_0x74(),
                    0x75 => self.ld_hl_l_0x75(),
                    0x76 => self.halt_0x76(),
                    0x77 => self.ld_hl_a_0x77(),
                    0x78 => self.ld_a_b_0x78(),
                    0x79 => self.ld_a_c_0x79(),
                    0x7A => self.ld_a_d_0x7a(),
                    0x7B => self.ld_a_e_0x7b(),
                    0x7C => self.ld_a_h_0x7c(),
                    0x7D => self.ld_a_l_0x7d(),
                    0x7E => self.ld_a_hl_0x7e(),
                    0x7F => self.ld_a_a_0x7f(),
                    // 0x8X
                    0x80 => self.add_a_b_0x80(),
                    0x81 => self.add_a_c_0x81(),
                    0x82 => self.add_a_d_0x82(),
                    0x83 => self.add_a_e_0x83(),
                    0x84 => self.add_a_h_0x84(),
                    0x85 => self.add_a_l_0x85(),
                    0x86 => self.add_a_hl_0x86(),
                    0x87 => self.add_a_a_0x87(),
                    0x88 => self.adc_a_b_0x88(),
                    0x89 => self.adc_a_c_0x89(),
                    0x8A => self.adc_a_d_0x8a(),
                    0x8B => self.adc_a_e_0x8b(),
                    0x8C => self.adc_a_h_0x8c(),
                    0x8D => self.adc_a_l_0x8d(),
                    0x8E => self.adc_a_hl_0x8e(),
                    0x8F => self.adc_a_a_0x8f(),

                    // 0x9X

                    0x90 => self.sub_b_0x90(),
                    0x91 => self.sub_c_0x91(),
                    0x92 => self.sub_d_0x92(),
                    0x93 => self.sub_e_0x93(),
                    0x94 => self.sub_h_0x94(),
                    0x95 => self.sub_l_0x95(),
                    0x96 => self.sub_hl_0x96(),
                    0x97 => self.sub_a_0x97(),
                    0x98 => self.sbc_a_b_0x98(),
                    0x99 => self.sbc_a_c_0x99(),
                    0x9A => self.sbc_a_d_0x9a(),
                    0x9B => self.sbc_a_e_0x9b(),
                    0x9C => self.sbc_a_h_0x9c(),
                    0x9D => self.sbc_a_l_0x9d(),
                    0x9E => self.sbc_a_hl_0x9e(),
                    0x9F => self.sbc_a_a_0x9f(),


                    // 0xAX
                    0xA0 => self.and_b_0xa0(),
                    0xA1 => self.and_c_0xa1(),
                    0xA2 => self.and_d_0xa2(),
                    0xA3 => self.and_e_0xa3(),
                    0xA4 => self.and_h_0xa4(),
                    0xA5 => self.and_l_0xa5(),
                    0xA6 => self.and_hl_0xa6(),
                    0xA7 => self.and_a_0xa7(),
                    0xA8 => self.xor_b_0xa8(),
                    0xA9 => self.xor_c_0xa9(),
                    0xAA => self.xor_d_0xaa(),
                    0xAB => self.xor_e_0xab(),
                    0xAC => self.xor_h_0xac(),
                    0xAD => self.xor_l_0xad(),
                    0xAE => self.xor_hl_0xae(),
                    0xAF => self.xor_a_0xaf(),

                    // 0xBX

                    0xB0 => self.or_b_0xb0(),
                    0xB1 => self.or_c_0xb1(),
                    0xB2 => self.or_d_0xb2(),
                    0xB3 => self.or_e_0xb3(),
                    0xB4 => self.or_h_0xb4(),
                    0xB5 => self.or_l_0xb5(),
                    0xB6 => self.or_hl_0xb6(),
                    0xB7 => self.or_a_0xb7(),
                    0xB8 => self.cp_b_0xb8(),
                    0xB9 => self.cp_c_0xb9(),
                    0xBA => self.cp_d_0xba(),
                    0xBB => self.cp_e_0xbb(),
                    0xBC => self.cp_h_0xbc(),
                    0xBD => self.cp_l_0xbd(),
                    0xBE => self.cp_hl_0xbe(),
                    0xBF => self.cp_a_0xbf(),

                    // 0xCX
                    0xC0 => self.ret_nz_0xc0(),
                    0xC1 => self.pop_bc_0xc1(),
                    0xC2 => self.jp_nz_a16_0xc2(),
                    0xC3 => self.jp_a16_0xc3(),
                    0xC4 => self.call_nz_a16_0xc4(),
                    0xC5 => self.push_bc_0xc5(),
                    0xC6 => self.add_a_d8_0xc6(),
                    0xC7 => self.rst_00h_0xc7(),
                    0xC8 => self.ret_z_0xc8(),
                    0xC9 => self.ret_0xc9(),
                    0xCA => self.jp_z_a16_0xca(),
                    0xCB => self.cb(),
                    0xCC => self.call_z_a16_0xcc(),
                    0xCD => self.call_a16_0xcd(),
                    0xCE => self.call_adc_a_d8_0xce(),
                    0xCF => self.rst_08h_0xcf(),

                    // 0xDX
                    0xD0 => self.ret_nc_0xd0(),
                    0xD1 => self.pop_de_0xd1(),
                    0xD2 => self.jp_nz_a6_0xd2(),
                    0xD4 => self.call_nc_a16_0xd4(),
                    0xD5 => self.push_de_0xd5(),
                    0xD6 => self.sub_d8_0xd6(),
                    0xD7 => self.rst_10h_0xd7(),
                    0xD8 => self.ret_c_0xd8(),
                    0xD9 => self.ret_0xd9(),
                    0xDA => self.jp_c_a16_0xda(),
                    0xDC => self.call_c_a16_0xdc(),
                    0xDE => self.sbc_a_d8_0xde(),
                    0xDF => self.rst_18h_0xdf(),

                    // 0xEX
                    0xE0 => self.ldh_a8_a_0xe0(),
                    0xE1 => self.pop_hl_0xe1(),
                    0xE2 => self.ld_c_a_0xe2(),
                    0xE5 => self.push_hl_0xe5(),
                    0xE6 => self.and_d8_0xe6(),
                    0xE7 => self.rst_20h_0xe7(),
                    0xE8 => self.add_sp_r8_0xe8(),
                    0xE9 => self.jp_hl_0xe9(),
                    0xEA => self.ld_a16_a_0xea(),
                    0xEE => self.xor_d8_0xee(),
                    0xEF => self.rst_28h_0xef(),

                    // 0xFX
                    0xF0 => self.ldh_a_a8_0xf0(),
                    0xF1 => self.pop_af_0xf1(),
                    0xF2 => self.ld_a_c_0xf2(),
                    0xF3 => self.di_0xf3(),
                    0xF5 => self.push_af_0xf5(),
                    0xF6 => self.or_d8_0xf6(),
                    0xF7 => self.rst_30h_0xf7(),
                    0xF8 => self.ld_hl_sp_s8_0xf8(),
                    0xF9 => self.ld_sp_hl_0xf9(),
                    0xFA => self.ld_a_a16_0xfa(),
                    0xFB => self.ei_0xfb(),
                    0xFE => self.cp_d8_0xfe(),
                    0xFF => self.rst_30h_0xff(),

                    // Unmapped code in default table
                    _ => panic!("Unmapped default table opcode {}", code),
                }
            }

            // CB Prefix Table
            true => {
                // Can safely unset the flag and interpret the *next* instruction normally
                self.use_cb_table = false;

                match code {
                    // 0xCB0X
                    0x00 => self.rlc_b_0xcb00(),
                    0x01 => self.rlc_c_0xcb01(),
                    0x02 => self.rlc_d_0xcb02(),
                    0x03 => self.rlc_e_0xcb03(),
                    0x04 => self.rlc_h_0xcb04(),
                    0x05 => self.rlc_l_0xcb05(),
                    0x06 => self.rlc_hl_0xcb06(),
                    0x07 => self.rlc_a_0xcb07(),
                    0x08 => self.rrc_b_0xcb08(),
                    0x09 => self.rrc_c_0xcb09(),
                    0x0A => self.rrc_d_0xcb0a(),
                    0x0B => self.rrc_e_0xcb0b(),
                    0x0C => self.rrc_h_0xcb0c(),
                    0x0D => self.rrc_l_0xcb0d(),
                    0x0E => self.rrc_hl_0xcb0e(),
                    0x0F => self.rrc_a_0xcb0f(),

                    // 0xCB1X
                    0x10 => self.rl_b_0xcb10(),
                    0x11 => self.rl_c_0xcb11(),
                    0x12 => self.rl_d_0xcb12(),
                    0x13 => self.rl_e_0xcb13(),
                    0x14 => self.rl_h_0xcb14(),
                    0x15 => self.rl_l0xcb15(),
                    0x16 => self.rl_hl_0xcb16(),
                    0x17 => self.rl_a_0xcb17(),

                    // 0xCB2X
                    0x20 => self.sla_b_0xcb20(),
                    0x21 => self.sla_c_0xcb21(),
                    0x22 => self.sla_d_0xcb22(),
                    0x23 => self.sla_e_0xcb23(),
                    0x24 => self.sla_h_0xcb24(),
                    0x25 => self.sla_l_0xcb25(),
                    0x26 => self.sla_hl_0xcb26(),
                    0x27 => self.sla_a_0xcb27(),
                    0x28 => self.sra_b_0xcb28(),
                    0x29 => self.sra_c_0xcb29(),
                    0x2A => self.sra_d_0xcb2a(),
                    0x2B => self.sra_e_0xcb2b(),
                    0x2C => self.sra_h_0xcb2c(),
                    0x2D => self.sra_l_0xcb2d(),
                    0x2E => self.sra_hl_0xcb2e(),
                    0x2F => self.sra_a_0xcb2f(),

                    // 0xCB3X
                    // TODO

                    // 0xCB4X
                    0x40 => self.bit_0_b_0xcb40(),
                    0x41 => self.bit_0_c_0xcb41(),
                    0x42 => self.bit_0_d_0xcb42(),
                    0x43 => self.bit_0_e_0xcb43(),
                    0x44 => self.bit_0_h_0xcb44(),
                    0x45 => self.bit_0_l_0xcb45(),
                    0x46 => self.bit_0_hl_0xcb46(),
                    0x47 => self.bit_0_a_0xcb47(),
                    0x48 => self.bit_1_b_0xcb48(),
                    0x49 => self.bit_1_c_0xcb49(),
                    0x4A => self.bit_1_d_0xcb4a(),
                    0x4B => self.bit_1_e_0xcb4b(),
                    0x4C => self.bit_1_h_0xcb4c(),
                    0x4D => self.bit_1_l_0xcb4d(),
                    0x4E => self.bit_1_hl_0xcb4e(),
                    0x4F => self.bit_1_a_0xcb4f(),

                    // 0xCB5X
                    // TODO

                    // 0xCB6X
                    0x60 => self.bit_2_b_0xcb60(),
                    0x61 => self.bit_2_c_0xcb61(),
                    0x62 => self.bit_2_d_0xcb62(),
                    0x63 => self.bit_2_e_0xcb63(),
                    0x64 => self.bit_2_h_0xcb64(),
                    0x65 => self.bit_2_l_0xcb65(),
                    0x66 => self.bit_2_hl_0xcb66(),
                    0x67 => self.bit_2_a_0xcb67(),
                    0x68 => self.bit_3_b_0xcb68(),
                    0x69 => self.bit_3_c_0xcb69(),
                    0x6A => self.bit_3_d_0xcb6a(),
                    0x6B => self.bit_3_e_0xcb6b(),
                    0x6C => self.bit_3_h_0xcb6c(),
                    0x6D => self.bit_3_l_0xcb6d(),
                    0x6E => self.bit_3_hl_0xcb6e(),
                    0x6F => self.bit_3_a_0xcb6f(),

                    // 0xCB7X
                    // TODO

                    // 0xCB8X
                    0x80 => self.res_0_b_0xcb80(),
                    0x81 => self.res_0_c_0xcb81(),
                    0x82 => self.res_0_d_0xcb82(),
                    0x83 => self.res_0_e_0xcb83(),
                    0x84 => self.res_0_h_0xcb84(),
                    0x85 => self.res_0_l_0xcb85(),
                    0x86 => self.res_0_hl_0xcb86(),
                    0x87 => self.res_0_a_0xcb87(),
                    0x88 => self.res_1_b_0xcb88(),
                    0x89 => self.res_1_c_0xcb89(),
                    0x8A => self.res_1_d_0xcb8a(),
                    0x8B => self.res_1_e_0xcb8b(),
                    0x8C => self.res_1_h_0xcb8c(),
                    0x8D => self.res_1_l_0xcb8d(),
                    0x8E => self.res_1_hl_0xcb8e(),
                    0x8F => self.res_1_a_0xcb8f(),

                    // 0xCB9X
                    // TODO

                    // 0xCBAX
                    0xA0 => self.res_4_b_0xcba0(),
                    0xA1 => self.res_4_c_0xcba1(),
                    0xA2 => self.res_4_d_0xcba2(),
                    0xA3 => self.res_4_e_0xcba3(),
                    0xA4 => self.res_4_h_0xcba4(),
                    0xA5 => self.res_4_l_0xcba5(),
                    0xA6 => self.res_4_hl_0xcba6(),
                    0xA7 => self.res_4_a_0xcba7(),
                    0xA8 => self.res_5_b_0xcba8(),
                    0xA9 => self.res_5_c_0xcba9(),
                    0xAA => self.res_5_d_0xcbaa(),
                    0xAB => self.res_5_e_0xcbab(),
                    0xAC => self.res_5_h_0xcbac(),
                    0xAD => self.res_5_l_0xcbad(),
                    0xAE => self.res_5_hl_0xcbae(),
                    0xAF => self.res_5_a_0xcbaf(),

                    // 0xCBBX
                    // TODO

                    // 0xCBCX
                    0xC0 => self.set_0_b_0xcbc0(),
                    0xC1 => self.set_0_c_0xcbc1(),
                    0xC2 => self.set_0_d_0xcbc2(),
                    0xC3 => self.set_0_e_0xcbc3(),
                    0xC4 => self.set_0_h_0xcbc4(),
                    0xC5 => self.set_0_l_0xcbc5(),
                    0xC6 => self.set_0_hl_0xcbc6(),
                    0xC7 => self.set_0_a_0xcbc7(),
                    0xC8 => self.set_1_b_0xcbc8(),
                    0xC9 => self.set_1_c_0xcbc9(),
                    0xCA => self.set_1_d_0xcbca(),
                    0xCB => self.set_1_e_0xcbcb(),
                    0xCC => self.set_1_h_0xcbcc(),
                    0xCD => self.set_1_l_0xcbcd(),
                    0xCE => self.set_1_hl_0xcbce(),
                    0xCF => self.set_1_a_0xcbcf(),

                    // 0xCBDX
                    // TODO

                    // 0xCBEX
                    0xE0 => self.set_4_b_0xcbe0(),
                    0xE1 => self.set_4_c_0xcbe1(),
                    0xE2 => self.set_4_d_0xcbe2(),
                    0xE3 => self.set_4_e_0xcbe3(),
                    0xE4 => self.set_4_h_0xcbe4(),
                    0xE5 => self.set_4_l_0xcbe5(),
                    0xE6 => self.set_4_hl_0xcbe6(),
                    0xE7 => self.set_4_a_0xcbe7(),
                    0xE8 => self.set_5_b_0xcbe8(),
                    0xE9 => self.set_5_c_0xcbe9(),
                    0xEA => self.set_5_d_0xcbea(),
                    0xEB => self.set_5_e_0xcbeb(),
                    0xEC => self.set_5_h_0xcbec(),
                    0xED => self.set_5_l_0xcbed(),
                    0xEE => self.set_5_hl_0xcbee(),
                    0xEF => self.set_5_a_0xcbef(),

                    // 0xCBFX
                    // TODO


                    // Unmapped code in default table; should be impossible as the table is complete
                    _ => panic!("Unmapped CB prefix opcode {}", code)
                }
            }
        }
    }

    /*****************************************/
    /*      Default 8-bit LR35902 Table      */
    /*****************************************/

    /*   0x00 - 0x0F   */

    // 0x00 - NOP
    pub fn nop_0x00(&mut self) -> u64 {
        4
    }

    // 0x01 - LD BC, d16
    fn ld_bc_0x01(&mut self) -> u64 {
        self.registers.c = self.byte();
        self.registers.b = self.byte();
        12
    }

    // 0x02 - LD (BC), A
    fn ld_bc_a_0x02(&mut self) -> u64 {
        self.mmu.write(self.registers.a, self.get_bc());
        8
    }

    // 0x03 - INC BC
    fn inc_bc_0x03(&mut self) -> u64 {
        let bc = self.inc_16(self.get_bc());
        self.set_bc(bc);
        8
    }

    // 0x04 - INC B
    fn inc_b_0x04(&mut self) -> u64 {
        self.registers.b = self.inc_8(self.registers.b);
        4
    }

    // 0x05 - DEC b
    fn dec_b_0x05(&mut self) -> u64 {
        self.registers.b = self.dec_8(self.registers.b);
        4
    }

    // 0x06 - LD B, d8
    fn ld_b_0x06(&mut self) -> u64 {
        self.registers.b = self.byte();
        8
    }

    // 0x07 - RLCA
    fn rlca_0x07(&mut self) -> u64 {
        self.registers.a = (self.registers.a << 1) | (self.registers.a >> 7);
        self.unset_zero();
        self.unset_subtraction();
        self.unset_half_carry();

        match self.registers.a & 0x80 {
            0x80 => self.set_full_carry(),
            _ => self.unset_full_carry(),
        };
        4
    }

    // 0x08 - LD (a16), SP
    fn ld_a16_sp_0x08(&mut self) -> u64 {
        let addr = self.word();
        self.mmu.write(self.registers.sp as u8, addr);
        self.mmu.write((self.registers.sp >> 8) as u8, addr + 1);
        20
    }

    // 0x09 - ADD HL, BC
    fn add_hl_bc_0x09(&mut self) -> u64 {
        let hl = self.get_hl();
        let bc = self.get_bc();
        let hl = self.add_16(hl, bc);
        self.set_hl(hl);
        8
    }

    // 0x0A - LD A, (BC)
    fn ld_a_bc_0x0a(&mut self) -> u64 {
        self.registers.a = self.mmu.read(self.get_bc());
        8
    }

    // 0x0B - DEC BC
    fn dec_bc_0x0b(&mut self) -> u64 {
        let bc = self.dec_16(self.get_bc());
        self.set_bc(bc);
        8
    }

    // 0x0C - INC C
    fn inc_c_0x0c(&mut self) -> u64 {
        self.registers.c = self.inc_8(self.registers.c);
        4
    }

    // 0x0D - DEC C
    fn dec_c_0x0d(&mut self) -> u64 {
        self.registers.c = self.dec_8(self.registers.c);
        4
    }

    // 0x0E - LD C, d8
    fn ld_c_0x0e(&mut self) -> u64 {
        self.registers.c = self.byte();
        8
    }

    // 0x0F - RRCA
    fn rrca_0x0f(&mut self) -> u64 {
        self.registers.a = (self.registers.a >> 1) | (self.registers.a << 7);
        self.unset_zero();
        self.unset_subtraction();
        self.unset_half_carry();
        match self.registers.a >> 7 != 0 {
            true => self.set_full_carry(),
            false => self.unset_full_carry(),
        };
        4
    }

    /*   0x10 - 0x1F   */

    /// 0x10 - STOP : Stops the system clock and oscillator circuit.
    /// LCD controller is also stopped.
    /// Internal RAM register ports remain unchanged
    /// Cancelled by RESET signal
    fn stop_0x10(&mut self) -> u64 {
        self.status = Status::STOPPED;
        4
    }

    /// 0x11 - LD DE, d16 : Loads 2 bytes of immediate data into registers D,E
    /// First byte is the lower byte, second byte is higher. Love Little endian -.-
    fn ld_de_0x11(&mut self) -> u64 {
        self.registers.d = self.byte();
        self.registers.e = self.byte();
        12
    }

    /// 0x12 - LD (DE), A : store contents of A in memory location specified by registers DE
    fn ld_de_a_0x12(&mut self) -> u64 {
        self.mmu.write(self.registers.a, self.get_de());
        8
    }

    /// 0x13 - INC DE : Increment the contents of registers DE by 1
    fn inc_de_0x13(&mut self) -> u64 {
        let de = self.inc_16(self.get_de());
        self.set_de(de);
        8
    }

    /// 0x14 - INC D : Increment the contents of D
    fn inc_d_0x14(&mut self) -> u64 {
        self.registers.d = self.inc_8(self.registers.d);
        4
    }

    /// 0x15 - DEC D: Decrement the register D
    fn dec_d_0x15(&mut self) -> u64 {
        self.registers.d = self.dec_8(self.registers.d);
        4
    }

    /// 0x16 - LD D, d8: Load the 8-bit immediate operand d8 into reg D
    fn ld_d_0x16(&mut self) -> u64 {
        self.registers.d = self.byte();
        8
    }

    ///0x17 - RLA : Rotate contents of register A to the left,
    fn rla_0x17(&mut self) -> u64 {
        self.unset_zero();
        self.unset_subtraction();
        self.unset_half_carry();
        let temp = self.is_full_carry();
        if self.registers.a & 0x80 == 1 {
            self.set_full_carry()
        } else {
            self.unset_full_carry()
        }
        self.registers.a = self.registers.a << 1;
        self.registers.a |= temp as u8;
        4
    }

    ///0x18 - JR s8 : Jump s8 steps from current address in program counter
    fn jr_s8_0x18(&mut self) -> u64 {
        let next = self.byte() as i8;
        self.jr(next);
        12
    }

    ///0x19 - ADD HL DE : add the contents of de to hl
    fn add_hl_de_0x19(&mut self) -> u64 {
        let val = self.add_16(self.get_hl(), self.get_de());
        self.set_hl(val);
        8
    }

    ///0x1A - LD A, (DE) : Load the 8-bit contents of memory specified by de into a
    fn ld_a_de_0x1a(&mut self) -> u64 {
        self.registers.a = self.mmu.read(self.get_de());
        8
    }

    /// 0x1B - DEC DE : decrement contents of de by 1!
    ///
    fn dec_de_0x1b(&mut self) -> u64 {
        let de = self.get_de();
        self.set_de(de);
        8
    }

    /// 0x1C - INC E
    fn inc_e_0x1c(&mut self) -> u64 {
        self.registers.e = self.inc_8(self.registers.e);
        4
    }

    ///0x1D - DEC E
    fn dec_e_0x1d(&mut self) -> u64 {
        self.registers.e = self.dec_8(self.registers.e);
        4
    }

    ///0x1E - LD E d8 : load 8 bit operand d8 into e
    fn ld_e_d8_0x1e(&mut self) -> u64 {
        self.registers.e = self.byte();
        8
    }

    ///0x1F - RRA : rotate register A to the right,
    /// through the carry flag,
    fn rra_0x1f(&mut self) -> u64 {
        let temp = self.is_full_carry();
        self.unset_zero();
        self.unset_subtraction();
        self.unset_half_carry();
        if self.registers.a & 0x01 != 0 {
            self.set_full_carry()
        } else {
            self.unset_full_carry()
        }
        self.registers.a = self.registers.a | (temp as u8) << 7;
        4
    }

    /*   0x20 - 0x2F   */

    // 0x20 - JR NZ s8
    fn jr_nz_s8_0x20(&mut self) -> u64 {
        let next = self.byte() as i8;

        match self.is_zero() {
            true => {
                self.jr(next);
                12
            }
            false => 8,
        }
    }

    // 0x21 - LD HL d16
    fn ld_hl_d16_0x21(&mut self) -> u64 {
        self.registers.h = self.byte();
        self.registers.l = self.byte();
        12
    }

    // 0x22 - LD (HL+) A
    fn ld_hlp_a_0x22(&mut self) -> u64 {
        let hl = self.get_hl();
        self.mmu.write(self.registers.a, hl);
        let hl = self.inc_16(hl);
        self.set_hl(hl);
        8
    }

    // 0x23 - INC HL
    fn inc_hl_0x23(&mut self) -> u64 {
        let hl = self.inc_16(self.get_hl());
        self.set_hl(hl);
        8
    }

    // 0x24 - INC H
    fn inc_h_0x24(&mut self) -> u64 {
        self.registers.h = self.inc_8(self.registers.h);
        4
    }

    // 0x25 - DEC H
    fn dec_h_0x25(&mut self) -> u64 {
        self.registers.h = self.dec_8(self.registers.h);
        4
    }

    // 0x26 - LD H d8
    fn ld_h_d8_0x26(&mut self) -> u64 {
        self.registers.h = self.byte();
        8
    }

    // 0x27 - DAA
    fn daa_0x27(&mut self) -> u64 {
        let mut adj = 0x00;

        if self.is_full_carry() {
            adj |= 0x60;
        }
        if self.is_half_carry() {
            adj |= 0x06;
        }

        if !self.is_subtraction() {
            if self.registers.a & 0x0F > 0x09 {
                adj |= 0x06;
            };
            if self.registers.a > 0x99 {
                adj |= 0x60;
            };
        }

        self.registers.a = self.registers.a.wrapping_add(adj);

        match adj >= 0x60 {
            true => self.set_full_carry(),
            false => self.unset_full_carry(),
        };

        self.unset_half_carry();

        match self.registers.a == 0 {
            true => self.set_zero(),
            false => self.unset_zero(),
        };

        4
    }

    // 0x28 - JR Z s8
    fn jr_z_s8_0x28(&mut self) -> u64 {
        let next = self.byte() as i8;

        match self.is_zero() {
            true => 8,
            false => {
                self.jr(next);
                12
            }
        }
    }

    // 0x29 - ADD HL HL
    fn add_hl_hl_0x29(&mut self) -> u64 {
        let hl = self.get_hl();
        let hl = self.add_16(hl, hl);
        self.set_hl(hl);
        8
    }

    // 0x2A LD A HL+
    fn ld_a_hlp_0x2a(&mut self) -> u64 {
        let hl = self.get_hl();
        self.registers.a = self.mmu.read(hl);
        let hl = self.inc_16(hl);
        self.set_hl(hl);
        8
    }

    // 0x2B - DEC HL
    fn dec_hl_0x2b(&mut self) -> u64 {
        let hl = self.dec_16(self.get_hl());
        self.set_hl(hl);
        8
    }

    // 0x2C - INC L
    fn inc_l_0x2c(&mut self) -> u64 {
        self.registers.l = self.inc_8(self.registers.l);
        4
    }

    // 0x2D - DEC L
    fn dec_l_0x2d(&mut self) -> u64 {
        self.registers.l = self.dec_8(self.registers.l);
        4
    }

    // 0x2E - LD L d8
    fn ld_l_d8_0x2e(&mut self) -> u64 {
        self.registers.l = self.byte();
        8
    }

    // 0x2F - CPL
    fn cpl_0x2f(&mut self) -> u64 {
        self.registers.a = !self.registers.a;
        4
    }

    /*   0x30 - 0x3F   */

    // 0x30 - JR NC, s8 : Jump s8 if carry flag is 0
    fn jr_nc_s8_0x30(&mut self) -> u64 {
        let next = self.byte() as i8;

        match !self.is_full_carry() {
            true => {
                self.jr(next);
                12
            }
            false => 8,
        }
    }
    // 0x31 - LD SP, d16 : Load the 2 bytes of immediate data into register pair SP
    fn ld_sp_d16_0x31(&mut self) -> u64 {
        self.registers.sp = self.word();
        12
    }

    // 0x32 - LD HL(-), A
    fn ld_hls_a_0x32(&mut self) -> u64 {
        self.mmu.write(self.registers.a, self.get_hl());
        let hl = self.dec_16(self.get_hl());
        self.set_hl(hl);
        8
    }

    // 0x33 - INC SP
    fn inc_sp_0x33(&mut self) -> u64 {
        self.registers.sp = self.inc_16(self.registers.sp);
        8
    }

    // 0x34 - INC (HL)
    fn inc_hl_0x34(&mut self) -> u64 {
        let mut hl = self.mmu.read(self.get_hl());
        hl = self.inc_8(hl);
        self.mmu.write(hl, self.get_hl());
        12
    }

    //0x35 - DEC (HL)
    fn dec_hl_0x35(&mut self) -> u64 {
        let mut hl = self.mmu.read(self.get_hl());
        hl = self.dec_8(hl);
        self.mmu.write(hl, self.get_hl());
        12
    }

    // 0x36 - LD HL, d8
    fn ld_hl_d8_0x36(&mut self) -> u64 {
        let d8 = self.byte();
        self.mmu.write(d8, self.get_hl());
        12
    }

    //0x37 - SCF
    fn scf_0x37(&mut self) -> u64 {
        self.set_full_carry();
        self.unset_half_carry();
        self.unset_subtraction();
        4
    }

    // 0x38 JR C, s8
    fn jr_c_s8_0x38(&mut self) -> u64 {
        match self.is_full_carry() {
            true => {
                let s8 = self.byte();
                self.jr(s8 as i8);
                12
            }
            false => 8,
        }
    }

    // 0x39 - ADD HL SP
    fn add_hl_sp_0x39(&mut self) -> u64 {
        let sp = self.registers.sp;
        let hl = self.add_16(self.get_hl(), sp);
        self.set_hl(hl);
        8
    }

    //0x3A - LD A, (HL-)
    fn ld_a_hls_0x3a(&mut self) -> u64 {
        let mut hl = self.get_hl();
        self.registers.a = self.mmu.read(hl);
        hl = self.inc_16(hl);
        self.set_hl(hl);
        8
    }

    // 0x3B - DEC SP
    fn dec_sp_0x3b(&mut self) -> u64 {
        self.registers.sp = self.dec_16(self.registers.sp);
        8
    }

    //0x3C - INC A
    fn inc_a_0x3c(&mut self) -> u64 {
        self.registers.a = self.inc_8(self.registers.a);
        4
    }

    //0x3D - DEC A
    fn dec_a_0x3d(&mut self) -> u64 {
        self.registers.a = self.dec_8(self.registers.a);
        4
    }

    //0x3E - LD A, d8
    fn ld_a_d8_0x3e(&mut self) -> u64 {
        self.registers.a = self.byte();
        8
    }

    // 0x3F - CCF
    fn ccf_0x3f(&mut self) -> u64 {
        match self.is_full_carry() {
            true => self.unset_full_carry(),
            false => self.set_full_carry(),
        };
        self.unset_subtraction();
        self.unset_half_carry();
        4
    }

    /*   0x40 - 0x4F   */
    // 0x40 - LD B B
    fn ld_b_b_0x40(&mut self) -> u64 {
        self.registers.b = self.registers.b; // ah, yes
        4
    }

    // 0x41 - LD B C
    fn ld_b_c_0x41(&mut self) -> u64 {
        self.registers.b = self.registers.c;
        4
    }

    // 0x42 - LD B D
    fn ld_b_d_0x42(&mut self) -> u64 {
        self.registers.b = self.registers.d;
        4
    }

    // 0x43 - LD B E
    fn ld_b_e_0x43(&mut self) -> u64 {
        self.registers.b = self.registers.e;
        4
    }

    // 0x44 - LD B H
    fn ld_b_h_0x44(&mut self) -> u64 {
        self.registers.b = self.registers.h;
        4
    }

    // 0x45 - LD B L
    fn ld_b_l_0x45(&mut self) -> u64 {
        self.registers.b = self.registers.l;
        4
    }

    // 0x46 - LD B (HL)
    fn ld_b_hl_0x46(&mut self) -> u64 {
        self.registers.b = self.mmu.read(self.get_hl());
        8
    }

    // 0x47 - LD B A
    fn ld_b_a_0x47(&mut self) -> u64 {
        self.registers.b = self.registers.a;
        4
    }

    // 0x48 - LD C B
    fn ld_c_b_0x48(&mut self) -> u64 {
        self.registers.c = self.registers.b;
        4
    }

    // 0x49 - LD C C
    fn ld_c_c_0x49(&mut self) -> u64 {
        self.registers.c = self.registers.c; // ok
        4
    }

    // 0x4A - LD C D
    fn ld_c_d_0x4a(&mut self) -> u64 {
        self.registers.c = self.registers.d;
        4
    }

    // 0x4B - LD C E
    fn ld_c_e_0x4b(&mut self) -> u64 {
        self.registers.c = self.registers.e;
        4
    }

    // 0x4C - LD C H
    fn ld_c_h_0x4c(&mut self) -> u64 {
        self.registers.c = self.registers.h;
        4
    }

    // 0x4D - LD C L
    fn ld_c_l_0x4d(&mut self) -> u64 {
        self.registers.c = self.registers.l;
        4
    }

    // 0x4E - LD C (HL)
    fn ld_c_hl_0x4e(&mut self) -> u64 {
        self.registers.c = self.mmu.read(self.get_hl());
        8
    }

    // 0x4F - LD C A
    fn ld_c_a_0x4f(&mut self) -> u64 {
        self.registers.c = self.registers.a;
        4
    }

    /*   0x50 - 0x5F   */

    // 0x50 - LD D, B
    fn ld_d_b_0x50(&mut self) -> u64 {
        self.registers.d = self.registers.b;
        4
    }

    // 0x51 - LD D, C
    fn ld_d_c_0x51(&mut self) -> u64 {
        self.registers.d = self.registers.c;
        4
    }

    // 0x52 - LD D, D
    fn ld_d_d_0x52(&mut self) -> u64 {
        self.registers.d = self.registers.d; //...
        4
    }

    // 0x53 - LD, D, E
    fn ld_d_e_0x53(&mut self) -> u64 {
        self.registers.d = self.registers.e;
        4
    }
    // 0x54 - LD, D, H
    fn ld_d_h_0x54(&mut self) -> u64 {
        self.registers.d = self.registers.h;
        4
    }

    // 0x55 - LD, D, L
    fn ld_d_l_0x55(&mut self) -> u64 {
        self.registers.d = self.registers.l;
        4
    }

    // 0x56 - LD D (HL)
    fn ld_d_hl_0x56(&mut self) -> u64 {
        self.registers.b = self.mmu.read(self.get_hl());
        8
    }

    // 0x57 - LD, D, A
    fn ld_d_a_0x57(&mut self) -> u64 {
        self.registers.d = self.registers.a;
        4
    }

    // 0x58 - LD, E, B
    fn ld_e_b_0x58(&mut self) -> u64 {
        self.registers.e = self.registers.b;
        4
    }

    // 0x59 - LD, E, C
    fn ld_e_c_0x59(&mut self) -> u64 {
        self.registers.e = self.registers.c;
        4
    }

    // 0x5A - LD, E, D
    fn ld_e_d_0x5a(&mut self) -> u64 {
        self.registers.e = self.registers.d;
        4
    }

    // 0x5B - LD, E, E
    fn ld_e_e_0x5b(&mut self) -> u64 {
        self.registers.e = self.registers.e; //literally nop
        4
    }

    // 0x5C - LD, E, H
    fn ld_e_h_0x5c(&mut self) -> u64 {
        self.registers.e = self.registers.h;
        4
    }

    // 0x5D - LD, E, L
    fn ld_e_l_0x5d(&mut self) -> u64 {
        self.registers.e = self.registers.l;
        4
    }

    // 0x5E - LD, E, HL
    fn ld_e_hl_0x5e(&mut self) -> u64 {
        self.registers.e = self.mmu.read(self.get_hl());
        8
    }

    // 0x5F - LD E, A
    fn ld_e_a_0x5f(&mut self) -> u64 {
        self.registers.e = self.registers.a;
        4
    }

    /*   0x60 - 0x6F   */

    // 0x60 - LD H B
    fn ld_h_b_0x60(&mut self) -> u64 {
        self.registers.h = self.registers.b;
        4
    }

    // 0x61 - LD H C
    fn ld_h_c_0x61(&mut self) -> u64 {
        self.registers.h = self.registers.c;
        4
    }

    // 0x62 - LD H D
    fn ld_h_d_0x62(&mut self) -> u64 {
        self.registers.h = self.registers.d;
        4
    }

    // 0x63 - LD H E
    fn ld_h_e_0x63(&mut self) -> u64 {
        self.registers.h = self.registers.e;
        4
    }

    // 0x64 - LD H H
    fn ld_h_h_0x64(&mut self) -> u64 {
        self.registers.h = self.registers.h; // sure
        4
    }

    // 0x65 - LD H L
    fn ld_h_l_0x65(&mut self) -> u64 {
        self.registers.h = self.registers.l;
        4
    }

    // 0x66 - LD H (HL)
    fn ld_h_hl_0x66(&mut self) -> u64 {
        self.registers.h = self.mmu.read(self.get_hl());
        8
    }

    // 0x67 - LD H A
    fn ld_h_a_0x67(&mut self) -> u64 {
        self.registers.h = self.registers.a;
        4
    }

    // 0x68 - LD L B
    fn ld_l_b_0x68(&mut self) -> u64 {
        self.registers.l = self.registers.b;
        4
    }

    // 0x69 - LD L C
    fn ld_l_c_0x69(&mut self) -> u64 {
        self.registers.l = self.registers.c;
        4
    }

    // 0x6A - LD L D
    fn ld_l_d_0x6a(&mut self) -> u64 {
        self.registers.l = self.registers.d;
        4
    }

    // 0x6B - LD L E
    fn ld_l_e_0x6b(&mut self) -> u64 {
        self.registers.l = self.registers.e;
        4
    }

    // 0x6C - LD L H
    fn ld_l_h_0x6c(&mut self) -> u64 {
        self.registers.l = self.registers.h;
        4
    }

    // 0x6D - LD L L
    fn ld_l_l_0x6d(&mut self) -> u64 {
        self.registers.l = self.registers.l; // ok
        4
    }

    // 0x6E - LD (HL)
    fn ld_l_hl_0x6e(&mut self) -> u64 {
        self.registers.l = self.mmu.read(self.get_hl());
        8
    }

    // 0x6F - LD L A
    fn ld_l_a_0x6f(&mut self) -> u64 {
        self.registers.l = self.registers.a;
        4
    }


    /*   0x70 - 0x7F   */

    // TODO - 0x70 - 0x7F

    /*   0x80 - 0x8F   */

    // 0x70 - LD (HL), B
    fn ld_hl_b_0x70(&mut self) -> u64 {
        self.mmu.write(self.registers.b, self.get_hl());
        8
    }

    // 0x71 - LD (HL), C
    fn ld_hl_c_0x71(&mut self) -> u64 {
        self.mmu.write(self.registers.c, self.get_hl());
        8
    }

    // 0x72 - LD (HL), D
    fn ld_hl_d_0x72(&mut self) -> u64 {
        self.mmu.write(self.registers.d, self.get_hl());
        8
    }

    // 0x73 - LD (HL), E
    fn ld_hl_e_0x73(&mut self) -> u64 {
        self.mmu.write(self.registers.e, self.get_hl());
        8
    }

    // 0x74 - LD (HL), H
    fn ld_hl_h_0x74(&mut self) -> u64 {
        self.mmu.write(self.registers.h, self.get_hl());
        8
    }

    // 0x75 - LD (HL), L
    fn ld_hl_l_0x75(&mut self) -> u64 {
        self.mmu.write(self.registers.l, self.get_hl());
        8
    }

    /// 0x76 - HALT : Stops the system clock\.
    /// Oscillator clock and LCD controller continue to operate.
    /// Internal RAM register ports remain unchanged
    /// Cancelled by interrupt or RESET signal
    ///
    fn halt_0x76(&mut self) -> u64 {
        self.status = Status::HALTED;
        4
    }

    // 0x77 - LD (HL), A
    fn ld_hl_a_0x77(&mut self) -> u64 {
        self.mmu.write(self.registers.a, self.get_hl());
        8
    }
    // 0x78 - LD, A, B
    fn ld_a_b_0x78(&mut self) -> u64 {
        self.registers.a = self.registers.b;
        4
    }

    //0x79 - LD A, C
    fn ld_a_c_0x79(&mut self) -> u64 {
        self.registers.a = self.registers.c;
        4
    }
    //0x7A - LD A, D
    fn ld_a_d_0x7a(&mut self) -> u64 {
        self.registers.a = self.registers.d;
        4
    }

    //0x7B - LD A, e
    fn ld_a_e_0x7b(&mut self) -> u64 {
        self.registers.a = self.registers.e;
        4
    }

    //0x7C - LD A, H
    fn ld_a_h_0x7c(&mut self) -> u64 {
        self.registers.a = self.registers.h;
        4
    }

    //0x7D - LD A, L
    fn ld_a_l_0x7d(&mut self) -> u64 {
        self.registers.a = self.registers.l;
        4
    }

    //0x7E - LD A, (HL)
    fn ld_a_hl_0x7e(&mut self) -> u64 {
        self.registers.a = self.mmu.read(self.get_hl());
        8
    }

    //0x7F - LD A, A
    fn ld_a_a_0x7f(&mut self) -> u64 {
        self.registers.a = self.registers.a; // I'll never get tired of these lol
        4
    }


    // 0x80 - ADD A,B
    fn add_a_b_0x80(&mut self) -> u64 {
        self.registers.a = self.add_8(self.registers.a, self.registers.b);
        4
    }

    // 0x81 - ADD A,C
    fn add_a_c_0x81(&mut self) -> u64 {
        self.registers.a = self.add_8(self.registers.a, self.registers.c);
        4
    }

    // 0x82 - ADD A,D
    fn add_a_d_0x82(&mut self) -> u64 {
        self.registers.a = self.add_8(self.registers.a, self.registers.d);
        4
    }

    // 0x83 - ADD A,E
    fn add_a_e_0x83(&mut self) -> u64 {
        self.registers.a = self.add_8(self.registers.a, self.registers.e);
        4
    }

    // 0x84 - ADD A,H
    fn add_a_h_0x84(&mut self) -> u64 {
        self.registers.a = self.add_8(self.registers.a, self.registers.h);
        4
    }

    // 0x85 - ADD A,L
    fn add_a_l_0x85(&mut self) -> u64 {
        self.registers.a = self.add_8(self.registers.a, self.registers.l);
        4
    }

    // 0x86 - ADD A,(HL)
    fn add_a_hl_0x86(&mut self) -> u64 {
        let value = self.mmu.read(self.get_hl());
        self.registers.a = self.add_8(self.registers.a, value);
        8
    }

    // 0x87 - ADD A,A
    fn add_a_a_0x87(&mut self) -> u64 {
        self.registers.a = self.add_8(self.registers.a, self.registers.a);
        4
    }

    // 0x88 - ADC A,B
    fn adc_a_b_0x88(&mut self) -> u64 {
        self.adc_8(self.registers.b);
        4
    }

    // 0x89 - ADC A,C
    fn adc_a_c_0x89(&mut self) -> u64 {
        self.adc_8(self.registers.c);
        4
    }

    // 0x8A - ADC A,D
    fn adc_a_d_0x8a(&mut self) -> u64 {
        self.adc_8(self.registers.d);
        4
    }

    // 0x8B - ADC A,E
    fn adc_a_e_0x8b(&mut self) -> u64 {
        self.adc_8(self.registers.e);
        4
    }

    // 0x8C - ADC A,H
    fn adc_a_h_0x8c(&mut self) -> u64 {
        self.adc_8(self.registers.h);
        4
    }

    // 0x8D - ADC A,L
    fn adc_a_l_0x8d(&mut self) -> u64 {
        self.adc_8(self.registers.l);
        4
    }

    // 0x8E - ADC A,(HL)
    fn adc_a_hl_0x8e(&mut self) -> u64 {
        let value = self.mmu.read(self.get_hl());
        self.adc_8(value);
        8
    }

    // 0x8F - ADC A,A
    fn adc_a_a_0x8f(&mut self) -> u64 {
        self.adc_8(self.registers.a);
        4
    }


    /*   0x90 - 0x9F   */

    // TODO - 0x90 - 0x9F

    /*   0xA0 - 0xAF   */

    // 0x90 - SUB B
    fn sub_b_0x90(&mut self) -> u64 {
        self.registers.a = self.sub_8(self.registers.a, self.registers.b);
        4
    }

    //0x91 - SUB C
    fn sub_c_0x91(&mut self) -> u64 {
        self.registers.a = self.sub_8(self.registers.a, self.registers.c);
        4
    }
    //0x92 - SUB D
    fn sub_d_0x92(&mut self) -> u64 {
        self.registers.a = self.sub_8(self.registers.a, self.registers.d);
        4
    }

    //0x93 - SUB E
    fn sub_e_0x93(&mut self) -> u64 {
        self.registers.a = self.sub_8(self.registers.a, self.registers.e);
        4
    }

    //0x94 - SUB H
    fn sub_h_0x94(&mut self) -> u64 {
        self.registers.a = self.sub_8(self.registers.a, self.registers.h);
        4
    }

    //0x95 - SUB L
    fn sub_l_0x95(&mut self) -> u64 {
        self.registers.a = self.sub_8(self.registers.a, self.registers.l);
        4
    }

    //0x96 - SUB (HL)
    fn sub_hl_0x96(&mut self) -> u64 {
        let val: u8 = self.mmu.read(self.get_hl());
        self.registers.a = self.sub_8(self.registers.a, val);
        8
    }

    //0x97 - SUB A
    fn sub_a_0x97(&mut self) -> u64 {
        self.registers.a = self.sub_8(self.registers.a, self.registers.a);
        4
    }

    //0x98 - SBC A, B
    fn sbc_a_b_0x98(&mut self) -> u64 {
        self.sbc_8(self.registers.b);
        4
    }

    //0x99 - SBC A, C
    fn sbc_a_c_0x99(&mut self) -> u64 {
        self.sbc_8(self.registers.c);
        4
    }

    //0x9A - SBC A, D
    fn sbc_a_d_0x9a(&mut self) -> u64 {
        self.sbc_8(self.registers.d);
        4
    }

    //0x9B - SBC A, E
    fn sbc_a_e_0x9b(&mut self) -> u64 {
        self.sbc_8(self.registers.e);
        4
    }

    //0x9C - SBC A, H
    fn sbc_a_h_0x9c(&mut self) -> u64 {
        self.sbc_8(self.registers.h);
        4
    }

    //0x9D - SBC A, L
    fn sbc_a_l_0x9d(&mut self) -> u64 {
        self.sbc_8(self.registers.l);
        4
    }

    //0x9E - SBC A, HL
    fn sbc_a_hl_0x9e(&mut self) -> u64 {
        let value = self.mmu.read(self.get_hl());
        self.sbc_8(value);
        4
    }

    //0x9F - SBC A, A
    fn sbc_a_a_0x9f(&mut self) -> u64 {
        self.sbc_8(self.registers.a);
        4
    }

    // 0xA0 - AND B
    fn and_b_0xa0(&mut self) -> u64 {
        self.and(self.registers.b);
        4
    }

    // 0xA1 - AND C
    fn and_c_0xa1(&mut self) -> u64 {
        self.and(self.registers.c);
        4
    }

    // 0xA2 - AND D
    fn and_d_0xa2(&mut self) -> u64 {
        self.and(self.registers.d);
        4
    }

    // 0xA3 - AND E
    fn and_e_0xa3(&mut self) -> u64 {
        self.and(self.registers.e);
        4
    }

    // 0xA4 - AND H
    fn and_h_0xa4(&mut self) -> u64 {
        self.and(self.registers.h);
        4
    }

    // 0xA5 - AND L
    fn and_l_0xa5(&mut self) -> u64 {
        self.and(self.registers.l);
        4
    }

    // 0xA6 - AND (HL)
    fn and_hl_0xa6(&mut self) -> u64 {
        let value = self.mmu.read(self.get_hl());
        self.and(value);
        8
    }

    // 0xA7 - AND A
    fn and_a_0xa7(&mut self) -> u64 {
        self.and(self.registers.a); // ok
        4
    }

    // 0xA8 - XOR B
    fn xor_b_0xa8(&mut self) -> u64 {
        self.xor(self.registers.b);
        4
    }

    // 0xA9 - XOR C
    fn xor_c_0xa9(&mut self) -> u64 {
        self.xor(self.registers.c);
        4
    }

    // 0xAA - XOR D
    fn xor_d_0xaa(&mut self) -> u64 {
        self.xor(self.registers.d);
        4
    }

    // 0xAB - XOR E
    fn xor_e_0xab(&mut self) -> u64 {
        self.xor(self.registers.e);
        4
    }

    // 0xAC - XOR H
    fn xor_h_0xac(&mut self) -> u64 {
        self.xor(self.registers.h);
        4
    }

    // 0xAD - XOR L
    fn xor_l_0xad(&mut self) -> u64 {
        self.xor(self.registers.l);
        4
    }

    // 0xAE - XOR (HL)
    fn xor_hl_0xae(&mut self) -> u64 {
        let value = self.mmu.read(self.get_hl());
        self.xor(value);
        8
    }

    // 0xAF - XOR A
    fn xor_a_0xaf(&mut self) -> u64 {
        self.xor(self.registers.a); // why not
        4
    }

    // 0xB0 - OR B
    fn or_b_0xb0(&mut self) -> u64 {
        self.or(self.registers.b);
        4
    }

    // 0xB1 - OR C
    fn or_c_0xb1(&mut self) -> u64 {
        self.or(self.registers.c);
        4
    }

    // 0xB2 - OR D
    fn or_d_0xb2(&mut self) -> u64 {
        self.or(self.registers.d);
        4
    }

    // 0xB3 - OR E
    fn or_e_0xb3(&mut self) -> u64 {
        self.or(self.registers.e);
        4
    }
    // 0xB4 - OR H
    fn or_h_0xb4(&mut self) -> u64 {
        self.or(self.registers.h);
        4
    }

    // 0xB5 - OR L
    fn or_l_0xb5(&mut self) -> u64 {
        self.or(self.registers.l);
        4
    }

    // 0xB6 - OR (HL)
    fn or_hl_0xb6(&mut self) -> u64 {
        let val = self.mmu.read(self.get_hl());
        self.or(val);
        8
    }
    // 0xB7 - OR A
    fn or_a_0xb7(&mut self) -> u64 {
        self.or(self.registers.a);
        4
    }

    // 0xB8 - CP B
    fn cp_b_0xb8(&mut self) -> u64 {
        self.cp(self.registers.b);
        4
    }

    // 0xB9 - CP C
    fn cp_c_0xb9(&mut self) -> u64 {
        self.cp(self.registers.c);
        4
    }

    // 0xBA - CP D
    fn cp_d_0xba(&mut self) -> u64 {
        self.cp(self.registers.d);
        4
    }

    // 0xBB - CP E
    fn cp_e_0xbb(&mut self) -> u64 { // bay bay
        self.cp(self.registers.e);
        4
    }

    // 0xBC - CP H
    fn cp_h_0xbc(&mut self) -> u64 {
        self.cp(self.registers.h);
        4
    }

    // 0xBD - CP L
    fn cp_l_0xbd(&mut self) -> u64 {
        self.cp(self.registers.l);
        4
    }
    // 0xBE - CP (HL)
    fn cp_hl_0xbe(&mut self) -> u64 {
        let val = self.mmu.read(self.get_hl());
        self.cp(val);
        4
    }

    // 0xBF - CP A
    fn cp_a_0xbf(&mut self) -> u64 {
        self.cp(self.registers.a);
        4
    }

    /*   0xB0 - 0xBF   */

    // TODO - 0xB0 - 0xBF

    /*   0xC0 - 0xCF   */

    // 0xC0 - RET NZ
    fn ret_nz_0xc0(&mut self) -> u64 {
        match self.is_zero() {
            true => 8,
            false => {
                self.ret();
                20
            }
        }
    }

    // 0xC1 - POP BC
    fn pop_bc_0xc1(&mut self) -> u64 {
        let value = self.pop_sp();
        self.set_bc(value);
        12
    }

    // 0xC2 - JP NZ a16
    fn jp_nz_a16_0xc2(&mut self) -> u64 {
        let value = self.word();
        match self.is_zero() {
            false => {
                self.registers.pc = value;
                16
            }
            true => 12,
        }
    }

    // 0xC3 - JP a16
    fn jp_a16_0xc3(&mut self) -> u64 {
        let value = self.word();
        self.registers.pc = value;
        16
    }

    // 0xC4 - CALL NZ a16
    fn call_nz_a16_0xc4(&mut self) -> u64 {
        let value = self.word();
        match self.is_zero() {
            true => {
                self.call(value);
                24
            }
            false => 12,
        }
    }

    // 0xC5 - PUSH BC
    fn push_bc_0xc5(&mut self) -> u64 {
        self.push_sp(self.get_bc());
        16
    }

    // 0xC6 - ADD A d8
    fn add_a_d8_0xc6(&mut self) -> u64 {
        let value = self.byte();
        self.registers.a = self.add_8(self.registers.a, value);
        8
    }

    // 0xC7 - RST 00H
    fn rst_00h_0xc7(&mut self) -> u64 {
        self.rst(0x00);
        16
    }

    // 0xC8 - RET Z
    fn ret_z_0xc8(&mut self) -> u64 {
        match self.is_zero() {
            true => {
                self.ret();
                20

            },
            false => 8

            }
        }

    // 0xC9 - RET
    fn ret_0xc9(&mut self) -> u64 {
        self.ret();
        16
    }

    // 0xCA - JP Z a16
    fn jp_z_a16_0xca(&mut self) -> u64 {
        let a16 = self.word();
        match self.is_zero() {
            true => {
                self.registers.pc = a16;
                16
            }
            false => 12,
        }
    }

    // 0xCB - CB PREFIX
    fn cb(&mut self) -> u64 {
        self.use_cb_table = true;
        4
    }

    // 0xCC - CALL Z a16
    fn call_z_a16_0xcc(&mut self) -> u64 {
        let a16 = self.word();
        match self.is_zero() {
            true => {
                self.call(a16);
                24
            }
            false => 12,
        }
    }

    // 0xCD - CALL a16
    fn call_a16_0xcd(&mut self) -> u64 {
        let a16 = self.word();
        self.call(a16);
        24
    }

    // 0xCE - ADC A d8
    fn call_adc_a_d8_0xce(&mut self) -> u64 {
        let d8 = self.byte();
        self.adc_8(d8);
        8
    }

    // 0xCF - RST 08H
    fn rst_08h_0xcf(&mut self) -> u64 {
        self.rst(0x08);
        16
    }

    /*   0xD0 - 0xDF   */

    // 0xD0 - RET NC
    fn ret_nc_0xd0(&mut self) -> u64 {
        match self.is_full_carry() {
            true => 8,
            false => {
                self.ret();
                20
            }
        }
    }

    // 0xD1 - POP DE
    fn pop_de_0xd1(&mut self) -> u64 {
         let value = self.pop_sp();
        self.set_de(value);
         12
    }

    // 0xD2 - JP NZ, a16
    fn jp_nz_a6_0xd2(&mut self) -> u64{
        let value = self.word();
        match self.is_full_carry(){
            false => {self.registers.pc=value; 16},
            true => 12,
        }
    }

    // 0xD4 - Call NC, a16
    fn call_nc_a16_0xd4(&mut self) -> u64 {
        let val = self.word();
        match self.is_full_carry(){
            true => {
                self.call(val);
                24
            },
            false => 12
        }
    }

    // 0xD5 - PUSH DE
    fn push_de_0xd5(&mut self) -> u64 {
        self.push_sp(self.get_de());
        16
    }

    // 0xD6 - SUB d8
    fn sub_d8_0xd6(&mut self) -> u64 {
        let val = self.byte();
        self.registers.a = self.sub_8(self.registers.a, val);
        8
    }

    // 0xD7 - RST 10H
    fn rst_10h_0xd7(&mut self) -> u64 {
        self.rst(0x10);
        16
    }

    // 0xD8 - RET C
    fn ret_c_0xd8(&mut self) -> u64 {
        match self.is_full_carry() {
            true => {
                self.ret();
                20
            }
            false => 8,
        }
    }

    // 0xD9 - RETI
    fn ret_0xd9(&mut self) -> u64 {
        self.ret();
        self.set_ime();
        16
    }

    // 0xDA - JP C, a16
    fn jp_c_a16_0xda(&mut self) ->u64 {
        let a16 = self.word();
        match self.is_full_carry() {
            true => {
                self.registers.pc = a16;
                16
            }
            false => 12,
        }
    }

    // 0xDC - CALL C, a16
    fn call_c_a16_0xdc(&mut self) -> u64{
        let a16 = self.word();
        match self.is_full_carry(){
            true => {
                self.call(a16); 24
            }
            false => 12,
        }
    }

    // 0xDE - SBC A, d8
    fn sbc_a_d8_0xde(&mut self) -> u64 {
        let d8 = self.byte();
        self.sbc_8(d8);
        8
    }

    // 0xDF - RST 18H
    fn rst_18h_0xdf(&mut self) -> u64 {
        self.rst(0x18);
        16
    }
    /*   0xE0 - 0xEF   */
    // 0xE0 - LDH (a8) A
    fn ldh_a8_a_0xe0(&mut self) -> u64 {
        let a8 = self.byte();
        self.mmu.write(self.registers.a, 0xFF00 | a8 as u16);
        12
    }

    // 0xE1 - POP HL
    fn pop_hl_0xe1(&mut self) -> u64 {
        let value = self.pop_sp();
        self.set_hl(value);
        12
    }

    // 0xE2 - LD (C) A
    fn ld_c_a_0xe2(&mut self) -> u64 {
        self.mmu
            .write(self.registers.a, 0xFF00 | self.registers.c as u16);
        8
    }

    // 0xE5 - PUSH HL
    fn push_hl_0xe5(&mut self) -> u64 {
        self.push_sp(self.get_hl());
        16
    }

    // 0xE6 - AND d8
    fn and_d8_0xe6(&mut self) -> u64 {
        let d8 = self.byte();
        self.and(d8);
        8
    }

    // 0xE7 - RST 20H
    fn rst_20h_0xe7(&mut self) -> u64 {
        self.rst(0x20);
        16
    }

    // 0xE8 - ADD SP, r8
    fn add_sp_r8_0xe8(&mut self) -> u64 {
        let r8 = self.byte() as i8 as i16 as u16;
        self.registers.sp = self.add_16(self.registers.sp, r8);
        self.unset_zero();
        16
    }

    // 0xE9 - JP (HL)
    fn jp_hl_0xe9(&mut self) -> u64 {
        self.registers.pc = self.get_hl();
        4
    }

    // 0xEA - LD (a16) A
    fn ld_a16_a_0xea(&mut self) -> u64 {
        let a16 = self.word();
        self.mmu.write(self.registers.a, a16);
        16
    }

    // 0xEE - XOR d8
    fn xor_d8_0xee(&mut self) -> u64 {
        let d8 = self.byte();
        self.xor(d8);
        8
    }

    // 0xEF - RST 28H
    fn rst_28h_0xef(&mut self) -> u64 {
        self.rst(0x28);
        16
    }

    /*   0xF0 - 0xFF   */
    // 0xF0 - LDH A, (a8)
    fn ldh_a_a8_0xf0(&mut self) -> u64{
        let a8 = self.byte();
        self.registers.a = self.mmu.read( 0xFF00 | a8 as u16);
        12
    }

    // 0xF1 - POP AF
    fn pop_af_0xf1(&mut self) -> u64 {
        let val = self.pop_sp();
        self.set_af(val);
        12
    }

    // 0xF2 - LD A, (C)
    fn ld_a_c_0xf2(&mut self) -> u64 {
        self.registers.a = self.mmu.read(0xFF00 | self.registers.c as u16);
        8
    }

    // 0xF3 - DI
    fn di_0xf3(&mut self) -> u64{
        self.unset_ime();
        4
    }

    // 0xF5 - PUSH AF
    fn push_af_0xf5(&mut self) -> u64 {
        self.push_sp(self.get_af());
        16
    }

    // 0xF6 - OR d8
    fn or_d8_0xf6(&mut self) -> u64 {
        let d8 = self.byte();
        self.or(d8);
        8
    }

    // 0xF7 - RST 30H
    fn rst_30h_0xf7(&mut self) -> u64 {
        self.rst(0x30);
        16
    }

    // 0xF8 - LD HL, SP + s8
    fn ld_hl_sp_s8_0xf8(&mut self)-> u64{
        let s8 = self.byte();
        let hl = self.add_16(self.registers.sp, s8 as i16 as u16);
        self.set_hl(hl);
        self.unset_zero();
        12
    }

    // 0xF9 - LD SP, HL
    fn ld_sp_hl_0xf9(&mut self)-> u64{
        let contents = self.mmu.read(self.get_hl());
        self.registers.sp = contents as u16;
        8
    }

    // 0xFA - LD A, a16
    fn ld_a_a16_0xfa(&mut self) -> u64 {
        let a16 = self.word();
        self.registers.a = self.mmu.read(a16);
        16
    }

    // 0xFB - EI
    fn ei_0xfb(&mut self) -> u64 {
        self.set_ime();
        4
    }

    // 0xFE - CP d8
    fn cp_d8_0xfe(&mut self) -> u64{
        let d8 = self.byte();
        self.cp(d8);
        8
    }

    // 0xFF - RST 30H
    fn rst_30h_0xff(&mut self) -> u64 {
        self.rst(0x30);
        16
    }
    /*****************************************/
    /*         16-bit CB Prefix Table        */
    /*****************************************/

    /* 0xCB00 - 0xCB0F */

    // 0xCB00 - RLC B
    fn rlc_b_0xcb00(&mut self) -> u64 {
        self.registers.b = self.rlc(self.registers.b);
        8
    }

    // 0xCB01 - RLC C
    fn rlc_c_0xcb01(&mut self) -> u64 {
        self.registers.c = self.rlc(self.registers.c);
        8
    }

    // 0xCB02 - RLC D
    fn rlc_d_0xcb02(&mut self) -> u64 {
        self.registers.d = self.rlc(self.registers.d);
        8
    }

    // 0xCB03 - RLC E
    fn rlc_e_0xcb03(&mut self) -> u64 {
        self.registers.e = self.rlc(self.registers.e);
        8
    }

    // 0xCB04 - RLC H
    fn rlc_h_0xcb04(&mut self) -> u64 {
        self.registers.h = self.rlc(self.registers.h);
        8
    }

    // 0xCB05 - RLC L
    fn rlc_l_0xcb05(&mut self) -> u64 {
        self.registers.l = self.rlc(self.registers.l);
        8
    }

    // 0xCB06 - RLC (HL)
    fn rlc_hl_0xcb06(&mut self) -> u64 {
        let value = self.rlc(self.mmu.read(self.get_hl()));
        self.mmu.write(value, self.get_hl());
        16
    }

    // 0xCB07 - RLC A
    fn rlc_a_0xcb07(&mut self) -> u64 {
        self.registers.a = self.rlc(self.registers.a);
        8
    }

    // 0xCB08 - RRC B
    fn rrc_b_0xcb08(&mut self) -> u64 {
        self.registers.b = self.rrc(self.registers.b);
        8
    }

    // 0xCB09 - RRC C
    fn rrc_c_0xcb09(&mut self) -> u64 {
        self.registers.c = self.rrc(self.registers.c);
        8
    }

    // 0xCB0A - RRC D
    fn rrc_d_0xcb0a(&mut self) -> u64 {
        self.registers.d = self.rrc(self.registers.d);
        8
    }

    // 0xCB0B - RRC E
    fn rrc_e_0xcb0b(&mut self) -> u64 {
        self.registers.e = self.rrc(self.registers.e);
        8
    }

    // 0xCB0C - RRC H
    fn rrc_h_0xcb0c(&mut self) -> u64 {
        self.registers.h = self.rrc(self.registers.h);
        8
    }

    // 0xCB0D - RRC L
    fn rrc_l_0xcb0d(&mut self) -> u64 {
        self.registers.l = self.rrc(self.registers.l);
        8
    }

    // 0xCB0E - RRC (HL)
    fn rrc_hl_0xcb0e(&mut self) -> u64 {
        let value = self.rrc(self.mmu.read(self.get_hl()));
        self.mmu.write(value, self.get_hl());
        16
    }

    // 0xCB0F - RRC A
    fn rrc_a_0xcb0f(&mut self) -> u64 {
        self.registers.a = self.rrc(self.registers.a);
        8
    }

    /* 0xCB10 - 0xCB1F */

    // TODO - 0xCB10 - 0xCB1F
    // 0xCB10 - RL B
    fn rl_b_0xcb10(&mut self) -> u64 {
        self.rl(self.registers.b);
        8
    }

    // 0xCB11 - RL C
    fn rl_c_0xcb11(&mut self) -> u64 {
        self.rl(self.registers.c);
        8
    }

    // 0xCB12 - RL D
    fn rl_d_0xcb12(&mut self) -> u64 {
        self.rl(self.registers.d);
        8
    }

    // 0xCB13 - RL E
    fn rl_e_0xcb13(&mut self) -> u64 {
        self.rl(self.registers.e);
        8
    }

    // 0xCB14 - RL H
    fn rl_h_0xcb14(&mut self) -> u64 {
        self.rl(self.registers.h);
        8
    }

    // 0xCB15 - RL L
    fn rl_l0xcb15(&mut self) -> u64 {
        self.rl(self.registers.l);
        8
    }

    // 0xCB16 - RL HL
    fn rl_hl_0xcb16(&mut self) -> u64 {
        let val = self.rl(self.mmu.read(self.get_hl()));
        self.mmu.write(val,self.get_hl());
        16
    }

    // 0xCB17 - RL A
    fn rl_a_0xcb17(&mut self) -> u64 {
        self.rl(self.registers.a);
        8
    }
    /* 0xCB20 - 0xCB2F */

    // 0xCB20 - SLA B
    fn sla_b_0xcb20(&mut self) -> u64 {
        self.registers.b = self.sla(self.registers.b);
        8
    }

    // 0xCB21 - SLA C
    fn sla_c_0xcb21(&mut self) -> u64 {
        self.registers.c = self.sla(self.registers.c);
        8
    }

    // 0xCB22 - SLA D
    fn sla_d_0xcb22(&mut self) -> u64 {
        self.registers.d = self.sla(self.registers.d);
        8
    }

    // 0xCB23 - SLA E
    fn sla_e_0xcb23(&mut self) -> u64 {
        self.registers.e = self.sla(self.registers.e);
        8
    }

    // 0xCB24 - SLA H
    fn sla_h_0xcb24(&mut self) -> u64 {
        self.registers.h = self.sla(self.registers.h);
        8
    }

    // 0xCB25 - SLA L
    fn sla_l_0xcb25(&mut self) -> u64 {
        self.registers.l = self.sla(self.registers.l);
        8
    }

    // 0xCB26 - SLA (HL)
    fn sla_hl_0xcb26(&mut self) -> u64 {
        let value = self.sla(self.mmu.read(self.get_hl()));
        self.mmu.write(value, self.get_hl());
        16
    }

    // 0xCB27 - SLA A
    fn sla_a_0xcb27(&mut self) -> u64 {
        self.registers.a = self.sla(self.registers.a);
        8
    }

    // 0xCB28 - SRA B
    fn sra_b_0xcb28(&mut self) -> u64 {
        self.registers.b = self.sra(self.registers.b);
        8
    }

    // 0xCB29 - SRA C
    fn sra_c_0xcb29(&mut self) -> u64 {
        self.registers.c = self.sra(self.registers.c);
        8
    }

    // 0xCB2A - SRA D
    fn sra_d_0xcb2a(&mut self) -> u64 {
        self.registers.d = self.sra(self.registers.d);
        8
    }

    // 0xCB2B - RRC E
    fn sra_e_0xcb2b(&mut self) -> u64 {
        self.registers.e = self.sra(self.registers.e);
        8
    }

    // 0xCB2C - SRA H
    fn sra_h_0xcb2c(&mut self) -> u64 {
        self.registers.h = self.sra(self.registers.h);
        8
    }

    // 0xCB2D - SRA L
    fn sra_l_0xcb2d(&mut self) -> u64 {
        self.registers.l = self.sra(self.registers.l);
        8
    }

    // 0xCB2E - SRA (HL)
    fn sra_hl_0xcb2e(&mut self) -> u64 {
        let value = self.sra(self.mmu.read(self.get_hl()));
        self.mmu.write(value, self.get_hl());
        16
    }

    // 0xCB2F - SRA A
    fn sra_a_0xcb2f(&mut self) -> u64 {
        self.registers.a = self.sra(self.registers.a);
        8
    }

    /* 0xCB30 - 0xCB3F */

    // TODO 0xCB30 - 0xCB3F

    /* 0xCB40 - 0xCB4F */

    // 0xCB40 - BIT 0, B
    fn bit_0_b_0xcb40(&mut self) -> u64 {
        self.bit(self.registers.b, 0);
        8
    }

    // 0xCB41 - BIT 0, C
    fn bit_0_c_0xcb41(&mut self) -> u64 {
        self.bit(self.registers.c, 0);
        8
    }

    // 0xCB42 - BIT 0, D
    fn bit_0_d_0xcb42(&mut self) -> u64 {
        self.bit(self.registers.d, 0);
        8
    }

    // 0xCB43 - BIT 0, E
    fn bit_0_e_0xcb43(&mut self) -> u64 {
        self.bit(self.registers.e, 0);
        8
    }

    // 0xCB44 - BIT 0, H
    fn bit_0_h_0xcb44(&mut self) -> u64 {
        self.bit(self.registers.h, 0);
        8
    }

    // 0xCB45 - BIT 0, L
    fn bit_0_l_0xcb45(&mut self) -> u64 {
        self.bit(self.registers.l, 0);
        8
    }

    // 0xCB46 - BIT 0, (HL)
    fn bit_0_hl_0xcb46(&mut self) -> u64 {
        let value = self.mmu.read(self.get_hl());
        self.bit(value, 0);
        16
    }

    // 0xCB47 - BIT 0, A
    fn bit_0_a_0xcb47(&mut self) -> u64 {
        self.bit(self.registers.a, 0);
        8
    }

    // 0xCB48 - BIT 1, B
    fn bit_1_b_0xcb48(&mut self) -> u64 {
        self.bit(self.registers.b, 1);
        8
    }

    // 0xCB49 - BIT 1, C
    fn bit_1_c_0xcb49(&mut self) -> u64 {
        self.bit(self.registers.c, 1);
        8
    }

    // 0xCB4A - BIT 1, D
    fn bit_1_d_0xcb4a(&mut self) -> u64 {
        self.bit(self.registers.d, 1);
        8
    }

    // 0xCB4B - BIT 1, E
    fn bit_1_e_0xcb4b(&mut self) -> u64 {
        self.bit(self.registers.e, 1);
        8
    }

    // 0xCB4C - BIT 1, H
    fn bit_1_h_0xcb4c(&mut self) -> u64 {
        self.bit(self.registers.h, 1);
        8
    }

    // 0xCB4D - BIT 1, L
    fn bit_1_l_0xcb4d(&mut self) -> u64 {
        self.bit(self.registers.l, 1);
        8
    }

    // 0xCB4E - BIT 1, (HL)
    fn bit_1_hl_0xcb4e(&mut self) -> u64 {
        let value = self.mmu.read(self.get_hl());
        self.bit(value, 1);
        16
    }

    // 0xCB4F - BIT 1, A
    fn bit_1_a_0xcb4f(&mut self) -> u64 {
        self.bit(self.registers.a, 1);
        8
    }

    /* 0xCB50 - 0xCB5F */

    // TODO - 0xCB50 - 0xCB5F

    /* 0xCB60 - 0xCB6F */

    // 0xCB60 - BIT 2, B
    fn bit_2_b_0xcb60(&mut self) -> u64 {
        self.bit(self.registers.b, 2);
        8
    }

    // 0xCB61 - BIT 2, C
    fn bit_2_c_0xcb61(&mut self) -> u64 {
        self.bit(self.registers.c, 2);
        8
    }

    // 0xCB62 - BIT 2, D
    fn bit_2_d_0xcb62(&mut self) -> u64 {
        self.bit(self.registers.d, 2);
        8
    }

    // 0xCB63 - BIT 2, E
    fn bit_2_e_0xcb63(&mut self) -> u64 {
        self.bit(self.registers.e, 2);
        8
    }

    // 0xCB64 - BIT 2, H
    fn bit_2_h_0xcb64(&mut self) -> u64 {
        self.bit(self.registers.h, 2);
        8
    }

    // 0xCB65 - BIT 2, L
    fn bit_2_l_0xcb65(&mut self) -> u64 {
        self.bit(self.registers.l, 2);
        8
    }

    // 0xCB66 - BIT 2, (HL)
    fn bit_2_hl_0xcb66(&mut self) -> u64 {
        let value = self.mmu.read(self.get_hl());
        self.bit(value, 2);
        16
    }

    // 0xCB67 - BIT 2, A
    fn bit_2_a_0xcb67(&mut self) -> u64 {
        self.bit(self.registers.a, 2);
        8
    }

    // 0xCB68 - BIT 3, B
    fn bit_3_b_0xcb68(&mut self) -> u64 {
        self.bit(self.registers.b, 3);
        8
    }

    // 0xCB69 - BIT 3, C
    fn bit_3_c_0xcb69(&mut self) -> u64 {
        self.bit(self.registers.c, 3);
        8
    }

    // 0xCB6A - BIT 3, D
    fn bit_3_d_0xcb6a(&mut self) -> u64 {
        self.bit(self.registers.d, 3);
        8
    }

    // 0xCB6B - BIT 3, E
    fn bit_3_e_0xcb6b(&mut self) -> u64 {
        self.bit(self.registers.e, 3);
        8
    }

    // 0xCB6C - BIT 3, H
    fn bit_3_h_0xcb6c(&mut self) -> u64 {
        self.bit(self.registers.h, 3);
        8
    }

    // 0xCB6D - BIT 3, L
    fn bit_3_l_0xcb6d(&mut self) -> u64 {
        self.bit(self.registers.l, 3);
        8
    }

    // 0xCB6E - BIT 3, (HL)
    fn bit_3_hl_0xcb6e(&mut self) -> u64 {
        let value = self.mmu.read(self.get_hl());
        self.bit(value, 3);
        16
    }

    // 0xCB6F - BIT 3, A
    fn bit_3_a_0xcb6f(&mut self) -> u64 {
        self.bit(self.registers.a, 3);
        8
    }

    /* 0xCB70 - 0xCB7F */

    // TODO - 0xCB70 - 0xCB7F

    /* 0xCB80 - 0xCB8F */

    // 0xCB80 - RES 0, B
    fn res_0_b_0xcb80(&mut self) -> u64 {
        self.registers.b &= !1;
        8
    }

    // 0xCB81 - RES 0, C
    fn res_0_c_0xcb81(&mut self) -> u64 {
        self.registers.c &= !1;
        8
    }

    // 0xCB82 - RES 0, D
    fn res_0_d_0xcb82(&mut self) -> u64 {
        self.registers.d &= !1;
        8
    }

    // 0xCB83 - RES 0, E
    fn res_0_e_0xcb83(&mut self) -> u64 {
        self.registers.e &= !1;
        8
    }

    // 0xCB84 - RES 0, H
    fn res_0_h_0xcb84(&mut self) -> u64 {
        self.registers.h &= !1;
        8
    }

    // 0xCB85 - RES 0, L
    fn res_0_l_0xcb85(&mut self) -> u64 {
        self.registers.l &= !1;
        8
    }

    // 0xCB86 - RES 0, (HL)
    fn res_0_hl_0xcb86(&mut self) -> u64 {
        let value = self.mmu.read(self.get_hl()) & !1;
        self.mmu.write(value, self.get_hl());
        16
    }

    // 0xCB87 - RES 0, A
    fn res_0_a_0xcb87(&mut self) -> u64 {
        self.registers.a &= !1;
        8
    }

    // 0xCB88 - RES 1, B
    fn res_1_b_0xcb88(&mut self) -> u64 {
        self.registers.b &= !2;
        8
    }

    // 0xCB89 - RES 1, C
    fn res_1_c_0xcb89(&mut self) -> u64 {
        self.registers.c &= !2;
        8
    }

    // 0xCB8A - RES 1, D
    fn res_1_d_0xcb8a(&mut self) -> u64 {
        self.registers.d &= !2;
        8
    }

    // 0xCB8B - RES 1, E
    fn res_1_e_0xcb8b(&mut self) -> u64 {
        self.registers.e &= !2;
        8
    }

    // 0xCB8C - RES 1, H
    fn res_1_h_0xcb8c(&mut self) -> u64 {
        self.registers.h &= !2;
        8
    }

    // 0xCB8D - RES 1, L
    fn res_1_l_0xcb8d(&mut self) -> u64 {
        self.registers.l &= !2;
        8
    }

    // 0xCB8E - RES 1, (HL)
    fn res_1_hl_0xcb8e(&mut self) -> u64 {
        let value = self.mmu.read(self.get_hl()) & !2;
        self.mmu.write(value, self.get_hl());
        16
    }

    // 0xCB8F - RES 1, A
    fn res_1_a_0xcb8f(&mut self) -> u64 {
        self.registers.a &= !2;
        8
    }

    /* 0xCB90 - 0xCB9F */

    // TODO - 0xCB90 - 0xCB9F

    /* 0xCBA0 - 0xCBAF */

    // 0xCBA0 - RES 4, B
    fn res_4_b_0xcba0(&mut self) -> u64 {
        self.registers.b &= !8;
        8
    }

    // 0xCBA1 - RES 4, C
    fn res_4_c_0xcba1(&mut self) -> u64 {
        self.registers.c &= !8;
        8
    }

    // 0xCBA2 - RES 4, D
    fn res_4_d_0xcba2(&mut self) -> u64 {
        self.registers.d &= !8;
        8
    }

    // 0xCBA3 - RES 4, E
    fn res_4_e_0xcba3(&mut self) -> u64 {
        self.registers.e &= !8;
        8
    }

    // 0xCBA4 - RES 4, H
    fn res_4_h_0xcba4(&mut self) -> u64 {
        self.registers.h &= !8;
        8
    }

    // 0xCBA5 - RES 4, L
    fn res_4_l_0xcba5(&mut self) -> u64 {
        self.registers.l &= !8;
        8
    }

    // 0xCBA6 - RES 4, (HL)
    fn res_4_hl_0xcba6(&mut self) -> u64 {
        let value = self.mmu.read(self.get_hl()) & !8;
        self.mmu.write(value, self.get_hl());
        16
    }

    // 0xCBA7 - RES 4, A
    fn res_4_a_0xcba7(&mut self) -> u64 {
        self.registers.a &= !8;
        8
    }

    // 0xCBA8 - RES 5, B
    fn res_5_b_0xcba8(&mut self) -> u64 {
        self.registers.b &= !16;
        8
    }

    // 0xCBA9 - RES 5, C
    fn res_5_c_0xcba9(&mut self) -> u64 {
        self.registers.c &= !16;
        8
    }

    // 0xCBAA - RES 5, D
    fn res_5_d_0xcbaa(&mut self) -> u64 {
        self.registers.d &= !16;
        8
    }

    // 0xCBAB - RES 5, E
    fn res_5_e_0xcbab(&mut self) -> u64 {
        self.registers.e &= !16;
        8
    }

    // 0xCBAC - RES 5, H
    fn res_5_h_0xcbac(&mut self) -> u64 {
        self.registers.h &= !16;
        8
    }

    // 0xCBAD - RES 5, L
    fn res_5_l_0xcbad(&mut self) -> u64 {
        self.registers.l &= !16;
        8
    }

    // 0xCBAE - RES 5, (HL)
    fn res_5_hl_0xcbae(&mut self) -> u64 {
        let value = self.mmu.read(self.get_hl()) & !16;
        self.mmu.write(value, self.get_hl());
        16
    }

    // 0xCBAF - RES 5, A
    fn res_5_a_0xcbaf(&mut self) -> u64 {
        self.registers.a &= !16;
        8
    }

    /* 0xCBB0 - 0xCBBF */

    // TODO - 0xCBB0 - 0xCBBF

    /* 0xCBC0 - 0xCBCF */

    // 0xCBC0 - SET 0, B
    fn set_0_b_0xcbc0(&mut self) -> u64 {
        self.registers.b |= 1;
        8
    }

    // 0xCBC1 - SET 0, C
    fn set_0_c_0xcbc1(&mut self) -> u64 {
        self.registers.c |= 1;
        8
    }

    // 0xCBC2 - SET 0, D
    fn set_0_d_0xcbc2(&mut self) -> u64 {
        self.registers.d |= 1;
        8
    }

    // 0xCBC3 - SET 0, E
    fn set_0_e_0xcbc3(&mut self) -> u64 {
        self.registers.e |= 1;
        8
    }

    // 0xCBc4 - SET 0, H
    fn set_0_h_0xcbc4(&mut self) -> u64 {
        self.registers.h |= 1;
        8
    }

    // 0xCBC5 - SET 0, L
    fn set_0_l_0xcbc5(&mut self) -> u64 {
        self.registers.l |= 1;
        8
    }

    // 0xCBC6 - SET 0, (HL)
    fn set_0_hl_0xcbc6(&mut self) -> u64 {
        let value = self.mmu.read(self.get_hl()) | 1;
        self.mmu.write(value, self.get_hl());
        16
    }

    // 0xCBC7 - SET 0, A
    fn set_0_a_0xcbc7(&mut self) -> u64 {
        self.registers.a |= 1;
        8
    }

    // 0xCBC8 - SET 1, B
    fn set_1_b_0xcbc8(&mut self) -> u64 {
        self.registers.b |= 2;
        8
    }

    // 0xCBC9 - SET 1, C
    fn set_1_c_0xcbc9(&mut self) -> u64 {
        self.registers.c |= 2;
        8
    }

    // 0xCBCA - SET 1, D
    fn set_1_d_0xcbca(&mut self) -> u64 {
        self.registers.d |= 2;
        8
    }

    // 0xCBCB - SET 1, E
    fn set_1_e_0xcbcb(&mut self) -> u64 {
        self.registers.e |= 2;
        8
    }

    // 0xCBCC - SET 1, H
    fn set_1_h_0xcbcc(&mut self) -> u64 {
        self.registers.h |= 2;
        8
    }

    // 0xCBCD - SET 1, L
    fn set_1_l_0xcbcd(&mut self) -> u64 {
        self.registers.l |= 2;
        8
    }

    // 0xCBCE - SET 1, (HL)
    fn set_1_hl_0xcbce(&mut self) -> u64 {
        let value = self.mmu.read(self.get_hl()) | 2;
        self.mmu.write(value, self.get_hl());
        16
    }

    // 0xCBCF - SET 1, A
    fn set_1_a_0xcbcf(&mut self) -> u64 {
        self.registers.a |= 2;
        8
    }

    /* 0xCBD0 - 0xCBDF */

    // TODO - 0xCBD0 - 0xCBDF

    /* 0xCBE0 - 0xCBEF */

    // 0xCBE0 - SET 4, B
    fn set_4_b_0xcbe0(&mut self) -> u64 {
        self.registers.b |= 8;
        8
    }

    // 0xCBE1 - SET 4, C
    fn set_4_c_0xcbe1(&mut self) -> u64 {
        self.registers.c |= 8;
        8
    }

    // 0xCBE2 - SET 4, D
    fn set_4_d_0xcbe2(&mut self) -> u64 {
        self.registers.d |= 8;
        8
    }

    // 0xCBE3 - SET 4, E
    fn set_4_e_0xcbe3(&mut self) -> u64 {
        self.registers.e |= 8;
        8
    }

    // 0xCBE4 - SET 4, H
    fn set_4_h_0xcbe4(&mut self) -> u64 {
        self.registers.h |= 8;
        8
    }

    // 0xCBE5 - SET 4, L
    fn set_4_l_0xcbe5(&mut self) -> u64 {
        self.registers.l |= 8;
        8
    }

    // 0xCBE6 - SET 4, (HL)
    fn set_4_hl_0xcbe6(&mut self) -> u64 {
        let value = self.mmu.read(self.get_hl()) | 8;
        self.mmu.write(value, self.get_hl());
        16
    }

    // 0xCBE7 - SET 4, A
    fn set_4_a_0xcbe7(&mut self) -> u64 {
        self.registers.a |= 8;
        8
    }

    // 0xCBE8 - SET 5, B
    fn set_5_b_0xcbe8(&mut self) -> u64 {
        self.registers.b |= 16;
        8
    }

    // 0xCBE9 - SET 5, C
    fn set_5_c_0xcbe9(&mut self) -> u64 {
        self.registers.c |= 16;
        8
    }

    // 0xCBEA - SET 5, D
    fn set_5_d_0xcbea(&mut self) -> u64 {
        self.registers.d |= 16;
        8
    }

    // 0xCBEB - SET 5, E
    fn set_5_e_0xcbeb(&mut self) -> u64 {
        self.registers.e |= 16;
        8
    }

    // 0xCBEC - SET 5, H
    fn set_5_h_0xcbec(&mut self) -> u64 {
        self.registers.h |= 16;
        8
    }

    // 0xCBED - SET 5, L
    fn set_5_l_0xcbed(&mut self) -> u64 {
        self.registers.l |= 16;
        8
    }

    // 0xCBEE - SET 5, (HL)
    fn set_5_hl_0xcbee(&mut self) -> u64 {
        let value = self.mmu.read(self.get_hl()) | 16;
        self.mmu.write(value, self.get_hl());
        16
    }

    // 0xCBEF - SET 5, A
    fn set_5_a_0xcbef(&mut self) -> u64 {
        self.registers.a |= 16;
        8
    }

    /* 0xCBF0 - 0xCBFF */

    // TODO - 0xCBF0 - 0xCBFF

}
