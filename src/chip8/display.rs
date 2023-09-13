extern crate sdl2;
use std::{process, thread, collections::HashMap};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub const CHIP8_HEIGHT: usize = 32;
pub const CHIP8_WIDTH: usize = 64;

pub const CHIP8_DISPLAY_HEIGHT: u32 = 320;
pub const CHIP8_DISPLAY_WIDTH: u32 = 640;

pub struct Display {
    data: [[u8; CHIP8_HEIGHT]; CHIP8_WIDTH],
    needs_refresh: bool,
    sdl_context: sdl2::Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,

    kb_map: HashMap<Keycode, u8>,
    keys: [bool; 16],
}

impl Display {
    pub fn new() -> Display {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("chip8 display", CHIP8_DISPLAY_WIDTH, CHIP8_DISPLAY_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        return Display {
            data: [[0; CHIP8_HEIGHT]; CHIP8_WIDTH],
            needs_refresh: true,
            sdl_context: sdl_context,
            canvas: canvas,

            // mapping from original chip8 to qwerty is as follows:
            // 1 2 3 C  ==>  1 2 3 4
            // 4 5 6 D  ==>  Q W E R
            // 7 8 9 E  ==>  A S D F
            // A 0 B F  ==>  Z X C V
            kb_map: HashMap::from([
                (Keycode::Kp1, 0x1),
                (Keycode::Kp2, 0x2),
                (Keycode::Kp3, 0x3),
                (Keycode::Kp4, 0xC),
                (Keycode::Q, 0x4),
                (Keycode::W, 0x5),
                (Keycode::E, 0x6),
                (Keycode::R, 0xD),
                (Keycode::A, 0x7),
                (Keycode::S, 0x8),
                (Keycode::D, 0x9),
                (Keycode::F, 0xE),
                (Keycode::Z, 0xA),
                (Keycode::X, 0x0),
                (Keycode::C, 0xB),
                (Keycode::V, 0xF),
                ]
            ),
            keys: [false; 16],
        };
    }

    pub fn read_pixel(&self, x: usize, y: usize) -> u8 {
        return self.data[x][y];
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, value: u8) {
        self.data[x][y] = value;
        self.needs_refresh = true;
    }

    pub fn refresh(&mut self) {
        if self.needs_refresh {
            self.canvas
                .set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
            self.canvas.clear();
            self.canvas
                .set_draw_color(sdl2::pixels::Color::RGB(57, 255, 0));
            for x in 0..CHIP8_WIDTH {
                for y in 0..CHIP8_HEIGHT {
                    if self.data[x][y] == 1 {
                        self.canvas
                            .fill_rect(sdl2::rect::Rect::new(
                                (x * 10) as i32,
                                (y * 10) as i32,
                                10,
                                10,
                            ))
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

    pub fn poll_keys(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            // grab relevant keys
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    process::exit(0);
                },
                Event::KeyDown { keycode: Some(key), .. } => {
                    if self.kb_map.contains_key(&key) {
                        let key_press = self.kb_map[&key];
                        self.key_down(key_press);
                    }
                },
                Event::KeyUp { keycode: Some(key), .. } => {
                    if self.kb_map.contains_key(&key) {
                        let key_press = self.kb_map[&key];
                        self.key_up(key_press);
                    }
                },
                _ => {}
            }
        }
    }

    pub fn key_up(&mut self, key: u8) {
        self.keys[key as usize] = false;
    }

    pub fn key_down(&mut self, key: u8) {
        self.keys[key as usize] = true;
    }

    pub fn is_key_down(&self, key: u8) -> bool {
        return self.keys[key as usize];
    }

}
