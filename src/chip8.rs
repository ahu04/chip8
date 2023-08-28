mod cpu;
mod ram;

pub struct Chip8 {
    ram: ram::Ram,
    cpu: cpu::Cpu,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        return Chip8 {
            ram: ram::Ram::new(),
            cpu: cpu::Cpu::new(),
        };
    }

    pub fn init_rom(&mut self, rom: &Vec<u8>) {
        self.ram.init_rom(rom);
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.step(&mut self.ram);
        }
    }
}
