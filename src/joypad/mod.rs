use crate::traits::RunComponent;
use sdl2::EventPump;

pub struct Joypad {
    event_pump: EventPump,
    joypad: u8,             // 0xFF00
    pub interrupt: bool     // This corresponds to bit 4 of the IF register at 0xFF0F
}

impl Joypad {

    pub fn new(event_pump: EventPump) -> Joypad {
        Joypad {
            event_pump,
            joypad: 0,
            interrupt: false
        }
    }

    pub fn read(&self) -> u8 {
        self.joypad
    }

    pub fn write(&mut self, value: u8) {
        self.joypad = value & 0x30;     // Only care about about bits 5, 4, which correspond to 0x30
    }

    // TODO sdl2 input to set bits for joypad
}


impl RunComponent for Joypad {
    fn run(&mut self, cpu_clock_cycles: u64) {

        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => std::process::exit(0),
                sdl2::event::Event::KeyDown { keycode, keymod, .. } => {
                    println!("keycode: {:?}, keymod: {}", keycode, keymod.bits());
                }
                _ => {}
            }
        }

        for key in self.event_pump.keyboard_state().pressed_scancodes().into_iter() {
            println!("{}", key);
        }
    }
}