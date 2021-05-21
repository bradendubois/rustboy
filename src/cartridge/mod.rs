pub struct Cartridge {

    rom: Vec<u8>,

    // Meta-data extracted from above ROM
    entry_point: usize,             // 0x0100 - 0x0103
    nintendo_logo: Vec<u8>,         // 0x0104 - 0x0133
    title: String,                  // 0x0134 - 0x0143
    manufacturer_code: String,      // 0x013F - 0x0142
    cgb_flag: u8,                   // 0x0143
    new_licensee_code: u16,         // 0x0144 - 0x0145
    sgb_flag: u8,                   // 0x0146
    cartidge_type: u8,              // 0x0147
    rom_size: u8,                   // 0x0148
    ram_size: u8,                   // 0x0149
    destination_code: u8,           // 0x014A
    old_licensee_code: u8,          // 0x014B
    mask_rom_version_number: u8,    // 0x014C
    header_checksum: u8,            // 0x014D
    global_checksum: u16            // 0x014E - 0x014F
}

impl Cartridge {

    pub fn new(&mut data: Vec<u8>) -> Cartridge {
        Cartridge {
            rom: data,
            entry_point: 0x0100,
            nintendo_logo: data[0x0104..0x0134],
            title: data[0x0134..0x0143],
            manufacturer_code: data[0x013F..0x0143],
            cgb_flag: data[0x0143],
            new_licensee_code: data[0x0144..0x0146],
            sgb_flag: data[0x0146],
            cartidge_type: data[0x0147],
            rom_size: data[0x0148],
            ram_size: data[0x0149],
            destination_code: data[0x014A],
            old_licensee_code: data[0x014B],
            mask_rom_version_number: data[0x014C],
            header_checksum: data[0x014D],
            global_checksum: data[0x014E..0x0150]
        }
    }
}