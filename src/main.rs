mod mmu;
mod lr35902;
mod cartridge;

use std::fs::read;




fn main() {

    // let file = "./roms/Tetris (World).gb";
    let file = "./roms/Tetris (World).gb";

    let cartridge = match read(file) {

        Ok(bytes) => {

            cartridge::Cartridge::new(bytes)

                /*
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

            let new_cartridge = cartridge::Cartridge::new(bytes);

            println!("{:?}", new_cartridge);

                 */

        },

        Err(err) => panic!("{}", err)
    };

    println!("{:?}", cartridge);

    let mut cpu = lr35902::LR35902::new(cartridge, true);

    println!("{:?}", cpu);

    cpu.run();
}
