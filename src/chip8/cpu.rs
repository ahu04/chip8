
use super::ram::Ram;
const RAM_START: u16 = 0x200;

#[derive(Default)]
pub struct Cpu {
    registers: [u8; 16],
    i: u16,
    dt: u8,
    st: u8,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
}

// high, x, y, low nibbles (4 bits each)
struct Instruction {
    h: u8,
    x: u8,
    y: u8,
    l: u8
}

impl Instruction {
    pub fn parse(v: u16) -> Instruction {
        return Instruction {
            h: ((v & 0xF000) >> 12) as u8,
            x: ((v & 0x0F00) >> 8) as u8,
            y: ((v & 0x00F0) >> 4) as u8,
            l: (v & 0x000F) as u8,
        };
    }
}
impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu = Cpu::default();
        cpu.pc = RAM_START;
        return cpu;
    }

    pub fn step(&mut self, ram: &mut Ram) {
        let mut hi = ram.read(self.pc) as u16;
        let mut lo = ram.read(self.pc + 1) as u16;
        let instr = Instruction::parse(hi << 8 | lo);
        println!("Instr: {:x} {:x} {:x} {:x}", instr.h, instr.x, instr.y, instr.l);
        self.pc += 2;
    }
}