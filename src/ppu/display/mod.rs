extern crate sdl2;

use self::sdl2::Sdl;
use self::sdl2::render::WindowCanvas;
use self::sdl2::rect::Point;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

pub struct Screen {
    context: Sdl,
    canvas: WindowCanvas,
    i: i16
}

impl Screen {

    pub fn new() -> Screen {

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

        canvas.present();

        Screen {
            context,
            canvas,
            i: 0
        }
    }

    pub fn run(&mut self) {
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.draw_point(Point::new(10, 10));
        self.canvas.present();
    }
}



