use crate::traits::RunComponent;
use sdl2::EventPump;
use sdl2::keyboard::{Keycode, Scancode};

// TODO - Configurable Keymap
const  D_PAD_DOWN: Keycode = Keycode::S;
const    D_PAD_UP: Keycode = Keycode::W;
const  D_PAD_LEFT: Keycode = Keycode::A;
const D_PAD_RIGHT: Keycode = Keycode::D;

const  START: Keycode = Keycode::F;
const SELECT: Keycode = Keycode::G;
const  BTN_B: Keycode = Keycode::J;
const  BTN_A: Keycode = Keycode::I;


pub struct Joypad {
    event_pump: EventPump,

    // Individual bits of  P1/JOYP - 0xFF00
    bit_5: bool,        // Select Action Buttons
    bit_4: bool,        // Select Direction Buttons
    bit_3: bool,        // Down  or Start
    bit_2: bool,        // Up    or Select
    bit_1: bool,        // Left  or B
    bit_0: bool,        // Right or A

    pub interrupt: bool     // This corresponds to bit 4 of the IF register at 0xFF0F
}

impl Joypad {

    pub fn new(event_pump: EventPump) -> Joypad {
        Joypad {
            event_pump,

            // Note - Bit 1 (True) = Not Pressed, 0 (False) = Pressed
            bit_5: true,
            bit_4: true,
            bit_3: true,
            bit_2: true,
            bit_1: true,
            bit_0: true,
            interrupt: false
        }
    }

    pub fn read(&self) -> u8 {
        let mut result = 0;

        if self.bit_5 { result |= 0x20 };
        if self.bit_4 { result |= 0x10 };
        if self.bit_3 { result |= 0x08 };
        if self.bit_2 { result |= 0x04 };
        if self.bit_1 { result |= 0x02 };
        if self.bit_0 { result |= 0x01 };

        result
    }

    pub fn write(&mut self, value: u8) {
        // Only care about about bits 5, 4, which correspond to 0x30
        self.bit_5 = value & 0x20 != 0;
        self.bit_4 = value & 0x10 != 0;
    }
}


impl RunComponent for Joypad {
    fn run(&mut self, _cpu_clock_cycles: u64) {
        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => std::process::exit(0),
                sdl2::event::Event::KeyDown { keycode, keymod, .. } => {
                    // println!("keycode: {:?}, keymod: {}", keycode, keymod.bits());
                }
                _ => {}
            }
        }

        let keys: std::collections::HashSet<Keycode> = self.event_pump.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();

        let bit_3_pressed = keys.contains(&D_PAD_DOWN) && self.bit_4 || keys.contains(&START) && self.bit_5;
        if self.bit_3 && bit_3_pressed { self.interrupt = true; }
        self.bit_3 = !bit_3_pressed;

        let bit_2_pressed = keys.contains(&D_PAD_UP) && self.bit_4 || keys.contains(&SELECT) && self.bit_5;
        if self.bit_2 && bit_2_pressed { self.interrupt = true; }
        self.bit_2 = !bit_2_pressed;

        let bit_1_pressed = keys.contains(&D_PAD_LEFT) && self.bit_4 || keys.contains(&BTN_B) && self.bit_5;
        if self.bit_1 && bit_1_pressed { self.interrupt = true; }
        self.bit_1 = !bit_1_pressed;

        let bit_0_pressed = keys.contains(&D_PAD_RIGHT) && self.bit_4 || keys.contains(&BTN_A) && self.bit_5;
        if self.bit_0 && bit_0_pressed { self.interrupt = true; }
        self.bit_0 = !bit_0_pressed;
    }
}