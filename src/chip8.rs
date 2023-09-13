mod cpu;
mod display;
mod ram;
use std::thread;
use std::time::Duration;

pub struct Chip8 {
    ram: ram::Ram,
    cpu: cpu::Cpu,
    display: display::Display,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        return Chip8 {
            ram: ram::Ram::new(),
            cpu: cpu::Cpu::new(),
            display: display::Display::new(),
        };
    }

    pub fn init_rom(&mut self, rom: &Vec<u8>) {
        self.ram.init_rom(rom);
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.step(&mut self.ram, &mut self.display);
            self.display.refresh();
            thread::sleep(Duration::from_millis(1));
        }
    }
}
