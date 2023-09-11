extern crate sdl2;

pub const CHIP8_HEIGHT: usize = 32;
pub const CHIP8_WIDTH: usize = 64;

pub const CHIP8_DISPLAY_HEIGHT: u32 = 320;
pub const CHIP8_DISPLAY_WIDTH: u32 = 640;

pub struct Display {
    data: [[u8; CHIP8_HEIGHT]; CHIP8_WIDTH],
    needs_refresh: bool,
    
    sdl_context: sdl2::Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Display {
    pub fn new() -> Display {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("chip8 display", CHIP8_DISPLAY_WIDTH, CHIP8_DISPLAY_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        return Display {
            data: [[0; CHIP8_HEIGHT]; CHIP8_WIDTH],
            needs_refresh: false,
            sdl_context : sdl_context,
            canvas: canvas, 
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: u8) {
        self.data[x][y] = value;
        self.needs_refresh = true;
    }

    pub fn refresh(&mut self) {
        if self.needs_refresh {
            self.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
            self.canvas.clear();

            self.canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
            for x in 0..CHIP8_WIDTH {
                for y in 0..CHIP8_HEIGHT {
                    if self.data[x][y] == 1 {
                        self.canvas.fill_rect(sdl2::rect::Rect::new((x * 10) as i32, (y * 10) as i32, 10, 10))
                        .unwrap();
                    }
                }
            }

            self.canvas.present();
            self.needs_refresh = false;
        }
    }
    
    pub fn clear(&mut self) {
        self.data = [[0; CHIP8_HEIGHT]; CHIP8_WIDTH];
        self.needs_refresh = true;
    }



}