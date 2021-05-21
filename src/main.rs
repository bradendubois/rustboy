mod mmu;
mod lr35902;
mod cartridge;

use std::fs::read;



fn main() {

    println!("Hello, world!");

    let file = "./roms/Tetris (World).gb";

    match read(file) {

        Ok(bytes) => {

            print!("{:?}", bytes);

            match bytes[0x147] {
                0x00 | 0x01 | 0x0F | 0x19 => println!("\n\nok {}", bytes[0x147]),
                _ =>  println!("\n\nbad {}", bytes[0x147])
            };

            match bytes[0x104] {
                0xCE => println!("OK"),
                _ => println!("NO")
            };

            println!("Cartridge!");

            let mut clone = bytes.clone();

            let new_cartridge = cartridge::Cartridge::new(clone);

            println!("{:?}", new_cartridge);

        },

        Err(err) => panic!("{}", err)
    };


}
