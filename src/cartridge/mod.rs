pub struct Cartridge {

    rom: Vec<u8>,

    entry_point: usize,
    nintendo_logo: Vec<u8>,
    title: String,
    manufacturer_code: String,
    cgb_flag: u8,
    new_licensee_code: (String, String),
    sgb_flag: u8,
    cartidge_type: u8,
    rom_size: u8,
    ram_size: u8,
    destination_code: u8,
    old_licensee_code: u8,
    mask_version_version_number: u8,
    header_checksum: u8,
    global_checksum: u16

}