extern crate sdl2;

// use self::sdl2::Sdl;
// use self::sdl2::render::WindowCanvas;
#[allow(unused_imports)]
use self::sdl2::rect::{Point};
use self::sdl2::pixels::Color;
use self::sdl2::VideoSubsystem;

#[allow(unused_imports)]
use self::sdl2::render::WindowCanvas;

#[allow(unused_imports)]
use super::{HEIGHT, WIDTH};

#[allow(unused)]
pub struct Screen {
    // canvas: WindowCanvas,
}

#[allow(unused)]
impl Screen {

    pub fn new(video_context: VideoSubsystem) -> Screen {

        /*
        let canvas = video_context
            .window("Rustboy", WIDTH as u32, HEIGHT as u32)
            .position_centered().opengl().build().unwrap()
            .into_canvas().build().unwrap();


         */

        Screen {
            // canvas
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



