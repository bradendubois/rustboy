extern crate sdl2;

use self::sdl2::Sdl;
use self::sdl2::render::WindowCanvas;
use self::sdl2::rect::{Point, Rect};
use self::sdl2::pixels::Color;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

pub struct Screen {
//    context: Sdl,
//    canvas: WindowCanvas,
    i: i16
}

impl Screen {

    pub fn new() -> Screen {
/*
        let context = sdl2::init().unwrap();
        let subsystem = context.video().unwrap();

        let window = subsystem
            .window(
                "RustBoy",
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
            )
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string()).unwrap();

        let mut canvas = window
            .into_canvas()
            .build()
            .unwrap();

        canvas.clear();     // comment out this one for some fun (and weird) canvases
        canvas.present();
*/
        Screen {
            // context,
            // canvas,
            i: 0
        }
    }

    pub fn draw(&mut self, pixels: Vec<(Point, Color)>) {

        /*
        for (point, color) in pixels.iter() {
            self.canvas.set_draw_color(*color);
            self.canvas.draw_point(*point);
        }

        self.canvas.present();

         */
    }
}



