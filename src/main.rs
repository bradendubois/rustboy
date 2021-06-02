mod mmu;
mod lr35902;
mod cartridge;
mod ppu;
mod sound;
mod timer;
mod joypad;
mod serial;

use std::{fs::read, env::args};
use std::process::exit;


const DEFAULT_ROM: &str = "./roms/Tetris (World).gb";

fn main() {

    let cli_args: Vec<String> = args().collect();

    let file = match cli_args.len() {
        1 => {
            println!("supply the path of a ROM to load");
            println!("defaulting to {}", DEFAULT_ROM);
            DEFAULT_ROM
        },
        2 => &cli_args[1],
        _ => {
            println!("ignoring additional args");
            println!("using {}", cli_args[1]);
            &cli_args[1]
        }
    };

    let cartridge = match read(file) {
        Ok(bytes) => cartridge::Cartridge::new(bytes),
        Err(err) => panic!("{}", err)
    };

    println!("{:?}", cartridge);

    let mut cpu = lr35902::LR35902::new(cartridge);

    println!("{:?}", cpu);

    cpu.run();

    exit(0);
}


#[cfg(test)]
mod tests {
    use std::io::Error;
    use crate::cartridge::Cartridge;
    use crate::lr35902::LR35902;
    use std::process::exit;

    #[test]
    fn roms() {

        // let rom_queue: Vec::new();

        // std::fs::read_dir("./")

        // std::env::current_dir().unwrap()?;

        let test_rom = "./roms/mooneye-gb_hwtests/acceptance/instr/daa.gb";

        match std::fs::read(test_rom) {
            Ok(b) => {
                let cartridge = Cartridge::new(b);
                let mut cpu = LR35902::new(cartridge);
                cpu.run();
            }

            Err(_) => panic!("bad")
        }

    }
}