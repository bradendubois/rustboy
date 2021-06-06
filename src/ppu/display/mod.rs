extern crate sdl2;

// use self::sdl2::Sdl;
// use self::sdl2::render::WindowCanvas;
use self::sdl2::rect::{Point};
use self::sdl2::pixels::Color;

// use super::{HEIGHT, WIDTH};

#[allow(unused)]
pub struct Screen {
    // context: Sdl,
    // canvas: WindowCanvas,
}

#[allow(unused)]
impl Screen {

    pub fn new() -> Screen {

        /*
        let context = sdl2::init().unwrap();
        let subsystem = context.video().unwrap();

        let window = subsystem
            .window(
                "RustBoy",
                WIDTH as u32,
                HEIGHT as u32,
            )
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string()).unwrap();

        let mut canvas = window
            .into_canvas()
            .build()
            .unwrap();

        // canvas.present();


         */
        Screen {
            // context,
            // canvas,
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



