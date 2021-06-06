use crate::cartridge::Cartridge;
use crate::lr35902::LR35902;


const MOONEYE: &str = "./roms/testing/mooneye";


/// Helper method to test with a Mooneye ROM
pub fn mooneye(path: &String) -> bool {

    println!("testing path: {}", path);

    let data = std::fs::read(path).unwrap();
    let cartridge = Cartridge::new(data);

    let mut cpu = LR35902::testing(cartridge);

    cpu.run();

    // Test success results in fibonacci sequence in registers
    return
        cpu.registers.b ==  3 &&
        cpu.registers.c ==  5 &&
        cpu.registers.d ==  8 &&
        cpu.registers.e == 13 &&
        cpu.registers.h == 21 &&
        cpu.registers.l == 34
}


pub fn mooneye_all(dir: &str) {

    let mut successful: Vec<String> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    let full_dir = &format!("{}/{}", MOONEYE, dir);

    for entry in std::fs::read_dir(full_dir).unwrap() {

        let path = entry.unwrap().path();
        let pathname = path.to_str().unwrap().to_string();

        if path.is_file() && pathname.ends_with(".gb") {
            match mooneye(&pathname) {
                true  => successful.push(pathname),
                false => errors.push(pathname)
            }
        }
    }

    if errors.len() == 0 {
        println!("{} : {} tests : all successful", dir, successful.len());
    } else {
        println!("{} - successful", dir);
        for success in successful.iter() {
            println!("  {}", success);
        }

        println!("{} - errors", dir);
        for error in errors.iter() {
            println!("  {}", error);
        }

        panic!("errors in {}", dir)
    }
}
