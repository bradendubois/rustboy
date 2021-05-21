

#[derive(Debug)]
pub struct Cartridge {

    rom: Vec<u8>,

    // Meta-data extracted from above ROM
    entry_point: usize,             // 0x0100 - 0x0103
    nintendo_logo: Vec<u8>,         // 0x0104 - 0x0133
    title: Box<str>,                // 0x0134 - 0x0143
    manufacturer_code: Box<str>,    // 0x013F - 0x0142
    cgb_flag: u8,                   // 0x0143
    new_licensee_code: u16,         // 0x0144 - 0x0145
    sgb_flag: u8,                   // 0x0146
    cartridge_type: u8,             // 0x0147
    rom_size: u8,                   // 0x0148
    ram_size: u8,                   // 0x0149
    destination_code: u8,           // 0x014A
    old_licensee_code: u8,          // 0x014B
    mask_rom_version_number: u8,    // 0x014C
    header_checksum: u8,            // 0x014D
    global_checksum: u16            // 0x014E - 0x014F
}

impl Cartridge {

    pub fn new(data: Vec<u8>) -> Cartridge {
        Cartridge {
            rom: data.clone(),
            entry_point: 0x0100,
            nintendo_logo: Vec::from(&data[0x0104..=0x0133]),
            title: Box::from(std::str::from_utf8(&data[0x0134 ..= 0x0142]).unwrap()),
            manufacturer_code: Box::from(std::str::from_utf8(&data[0x013F ..= 0x0142]).unwrap()),
            cgb_flag: data[0x0143],
            new_licensee_code: ((data[0x0144] as u16) << 8) | (data[0x0145] as u16),
            sgb_flag: data[0x0146],
            cartridge_type: data[0x0147],
            rom_size: data[0x0148],
            ram_size: data[0x0149],
            destination_code: data[0x014A],
            old_licensee_code: data[0x014B],
            mask_rom_version_number: data[0x014C],
            header_checksum: data[0x014D],
            global_checksum: ((data[0x014E] as u16) << 8) | data[0x0150] as u16
        }
    }
}