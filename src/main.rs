mod mmu;
mod lr35902;

use std::fs::{read, File};
use std::path::{Path, PathBuf};
use std::io::{Error, Read};

fn main() {

    println!("Hello, world!");

    let file = "./roms/Super Mario Land (JUE) (V1.1) [!].gb";
    let path = PathBuf::from(file);

    let mut data = vec![];

    match File::open(&path) {
        Ok(mut open) => {

            let contents = open.read_to_end(&mut data);
            print!("{:?}", data);

            match data[0x147] {
                0x00 | 0x01 | 0x0F | 0x19 => println!("\n\nok {}", data[0x147]),
                _ =>  println!("\n\nbad {}", data[0x147])
            }

            match data[0x104] {
                0xCE => println!("OK"),
                _ => println!("NO")
            }
        },

        Err(err) => panic!("{}", err)
    };



}
