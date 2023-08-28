use super::ram::Ram;
use rand::prelude::*;
const RAM_START: u16 = 0x200;

#[derive(Default)]
pub struct Cpu {
    regs: [u8; 16],
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
    l: u8,
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
        let raw_instr = hi << 8 | lo;
        let instr = Instruction::parse(raw_instr);
        let nnn = raw_instr & 0x0FFF;
        let kk = (raw_instr & 0x00FF) as u8;
        match instr {
            Instruction { h: 0, l: 0, .. } => self.cls(),
            Instruction { h: 0, l: 0xE, .. } => self.ret(),
            Instruction { h: 1, .. } => self.jump(nnn),
            Instruction { h: 2, .. } => self.call(nnn),
            Instruction { h: 3, .. } => self.skip(self.regs[instr.x as usize], kk, true),
            Instruction { h: 4, .. } => self.skip(self.regs[instr.x as usize], kk, false),
            Instruction { h: 5, .. } => self.skip(
                self.regs[instr.x as usize],
                self.regs[instr.y as usize],
                true,
            ),
            Instruction { h: 6, .. } => self.set_reg(instr.x, kk),
            Instruction { h: 7, .. } => self.set_reg(instr.x, self.regs[instr.x as usize] + kk),
            Instruction { h: 8, l: 0, .. } => self.set_reg(instr.x, self.regs[instr.y as usize]),
            Instruction { h: 8, l: 1, .. } => self.set_reg(
                instr.x,
                self.regs[instr.x as usize] | self.regs[instr.y as usize],
            ),
            Instruction { h: 8, l: 2, .. } => self.set_reg(
                instr.x,
                self.regs[instr.x as usize] & self.regs[instr.y as usize],
            ),
            Instruction { h: 8, l: 3, .. } => self.set_reg(
                instr.x,
                self.regs[instr.x as usize] ^ self.regs[instr.y as usize],
            ),
            Instruction { h: 8, l: 4, .. } => {
                let (res, carry) = add_with_carry(
                    self.regs[instr.x as usize] as u16,
                    self.regs[instr.y as usize] as u16,
                );
                self.set_reg(instr.x, res);
                self.regs[0xF] = carry;
            }
            Instruction { h: 8, l: 5, .. } => {
                let (res, borrow) = sub_with_borrow(
                    self.regs[instr.x as usize] as u16,
                    self.regs[instr.y as usize] as u16,
                );
                self.regs[0xF] = !borrow;
                self.set_reg(instr.x, res);
            }
            Instruction { h: 8, l: 6, .. } => {
                self.regs[0xF] = self.regs[instr.x as usize] & 0x1;
                self.set_reg(instr.x, self.regs[instr.x as usize] >> 1);
            }
            Instruction { h: 8, l: 7, .. } => {
                let (res, borrow) = sub_with_borrow(
                    self.regs[instr.y as usize] as u16,
                    self.regs[instr.x as usize] as u16,
                );
                self.regs[0xF] = !borrow;
                self.set_reg(instr.x, res);
            }
            Instruction { h: 8, l: 8, .. } => {
                self.regs[0xF] = self.regs[instr.x as usize] & 0x1;
                self.set_reg(instr.x, self.regs[instr.x as usize] << 1);
            }
            Instruction { h: 9, .. } => {
                self.skip(
                    self.regs[instr.x as usize],
                    self.regs[instr.y as usize],
                    false,
                );
            }
            Instruction { h: 0xA, .. } => {
                self.i = nnn;
                self.pc += 2;
            }
            Instruction { h: 0xB, .. } => {
                self.jump(nnn + self.regs[0] as u16);
            }
            Instruction { h: 0xC, .. } => {
                self.set_reg(instr.x, rand::random::<u8>() & kk);
            }
            Instruction { h: 0xD, .. } => {
                self.display_sprite(instr.x, instr.y, instr.l);
            }
            Instruction { h: 0xE, l: 0xE, .. } => {
                // todo
                println!("not implemented");
                self.pc += 2;
            }
            Instruction { h: 0xE, y: 0xA, .. } => {}
            Instruction {
                h: 0xF, y: 0, l: 7, ..
            } => {
                self.set_reg(instr.x, self.dt);
            }
            Instruction {
                h: 0xF,
                y: 0,
                l: 0xA,
                ..
            } => {
                println!("not implemented");
                self.pc += 2;
            }
            Instruction {
                h: 0xF, y: 1, l: 5, ..
            } => {
                self.dt = self.regs[instr.x as usize];
                self.pc += 2;
            }
            Instruction {
                h: 0xF, y: 1, l: 8, ..
            } => {
                self.st = self.regs[instr.x as usize];
                self.pc += 2;
            }
            Instruction {
                h: 0xF,
                y: 1,
                l: 0xE,
                ..
            } => {
                self.i = self.i + self.regs[instr.x as usize] as u16;
                self.pc += 2;
            }
            Instruction {
                h: 0xF, y: 2, l: 9, ..
            } => {
                // todo
                println!("not implemented");
                self.pc += 2;
            }
            Instruction {
                h: 0xF, y: 3, l: 3, ..
            } => {
                self.store_bcd(instr.x, ram);
            }
            Instruction {
                h: 0xF, y: 5, l: 5, ..
            } => {
                self.write_regs(instr.x, ram);
            }
            Instruction {
                h: 0xF, y: 6, l: 5, ..
            } => {
                self.read_regs(instr.x, ram);
            }
            _ => panic!("unknown instruction: {:x}", raw_instr),
        }
    }

    fn cls(&mut self) {
        // todo
        println!("cls");
        self.pc += 2;
    }

    fn ret(&mut self) {
        println!("ret");
        self.pc = self.stack[self.sp as usize];
        self.sp -= 1;
    }

    fn jump(&mut self, addr: u16) {
        println!("jump");
        self.pc = addr;
    }

    fn call(&mut self, addr: u16) {
        println!("call");
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;
        self.pc = addr;
    }

    // if c && a == b, skip or
    // if !c && a != b, skip, otherwise don't skip
    fn skip(&mut self, a: u8, b: u8, c: bool) {
        println!("conditional skip");
        if (c && a == b) || (!c && a != b) {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    fn set_reg(&mut self, i: u8, v: u8) {
        self.regs[i as usize] = v;
        self.pc += 2;
    }

    fn display_sprite(&mut self, x: u8, y: u8, n: u8) {
        // todo
        self.pc += 2;
    }

    fn store_bcd(&mut self, x: u8, ram: &mut Ram) {
        let vx = self.regs[x as usize];
        ram.write(self.i, vx / 100);
        ram.write(self.i + 1, (vx / 10) % 10);
        ram.write(self.i + 2, vx % 10);
        self.pc += 2;
    }

    fn write_regs(&mut self, x: u8, ram: &mut Ram) {
        for j in 0..=x {
            ram.write(self.i + j as u16, self.regs[j as usize]);
        }
        self.pc += 2;
    }

    fn read_regs(&mut self, x: u8, ram: &mut Ram) {
        for j in 0..=x {
            self.regs[j as usize] = ram.read(self.i + j as u16);
        }
        self.pc += 2;
    }
}

fn add_with_carry(a: u16, b: u16) -> (u8, u8) {
    let res = a + b;
    if res > 255 {
        return ((res & 0xFF) as u8, 1);
    } else {
        return (res as u8, 0);
    }
}

fn sub_with_borrow(a: u16, b: u16) -> (u8, u8) {
    let res = a - b;
    if res > 255 {
        return ((res & 0xFF) as u8, 1);
    } else {
        return (res as u8, 0);
    }
}
