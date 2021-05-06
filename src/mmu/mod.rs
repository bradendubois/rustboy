struct MMU{
    in_bios: bool,
    bios: Vec<u8>,
    rom: Vec<u16>,
    w_ram: Vec<u16>,
    e_ram: Vec<u16>,
    z_ram: Vec<u8>,
}

