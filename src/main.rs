mod mmu;
mod lr35902;
mod cartridge;
mod ppu;
mod sound;
mod timer;
mod joypad;
mod serial;

mod traits;
mod enums;

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
