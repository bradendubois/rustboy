use nix::unistd::{fork, ForkResult};

use crate::cartridge::Cartridge;
use crate::lr35902::LR35902;
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use std::process::exit;


/* The [allow(dead_code)] flags are misleading; the code here is used in testing, which can be
 *  done with either `cargo test` or `cargo test -- --nocapture`, but a basic `cargo check` would
 *  suggest that the contents are not used.
 */


#[allow(dead_code)]
const MOONEYE: &str = "./roms/testing/mooneye";


/// Helper method to test with a Mooneye ROM
#[allow(dead_code)]
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
        cpu.registers.l == 34;
}


#[allow(dead_code)]
pub fn mooneye_all(dir: &str) {

    let mut successful: Vec<String> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    let full_dir = &format!("{}/{}", MOONEYE, dir);

    for entry in std::fs::read_dir(full_dir).unwrap() {

        let path = entry.unwrap().path();
        let pathname = path.to_str().unwrap().to_string();

        if path.is_file() && pathname.ends_with(".gb") {

            match fork() {
                Ok(ForkResult::Parent { child, .. }) => {
                    let x = waitpid(child, None).unwrap();
                    match x {
                        WaitStatus::Exited(_, exit_code) => {
                            match exit_code == 0 {
                                true  => successful.push(pathname),
                                false => errors.push(pathname)
                            };
                        }
                        _ => panic!("process did not exit")
                    }
                },

                Ok(ForkResult::Child) => {
                    std::process::exit(if mooneye(&pathname) { 0 } else { 1 })
                }

                Err(_) => panic!("failed fork")
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
