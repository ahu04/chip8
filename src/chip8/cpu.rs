use super::display::{Display, CHIP8_HEIGHT, CHIP8_WIDTH};
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

    pub fn step(&mut self, ram: &mut Ram, display: &mut Display) {
        display.poll_keys();
        let mut hi = ram.read(self.pc) as u16;
        let mut lo = ram.read(self.pc + 1) as u16;
        let raw_instr = hi << 8 | lo;
        let instr = Instruction::parse(raw_instr);
        let nnn = raw_instr & 0x0FFF;
        let kk = (raw_instr & 0x00FF) as u8;
        match instr {
            Instruction { h: 0, l: 0, .. } => self.cls(display),
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
            Instruction { h: 8, l: E, .. } => {
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
                self.display_sprite(ram, display, instr.x, instr.y, instr.l);
            }
            Instruction { h: 0xE, l: 0xE, .. } => {
                if display.is_key_down(self.regs[instr.x as usize]) {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            Instruction { h: 0xE, l: 1, .. } => {
                if !display.is_key_down(self.regs[instr.x as usize]) {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
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
                let mut press: u8 = 0xFF;
                while (press == 0xFF) {
                    for i in 0..16 {
                        if display.is_key_down(i) {
                            press = i;
                            break;
                        }
                    }
                    display.poll_keys();
                }
                self.set_reg(instr.x, press);
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
                self.i = (self.regs[instr.x as usize] as u16) * 5;
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
        if (self.dt > 0) {
            self.dt -= 1;
        }
    }

    fn cls(&mut self, display : &mut Display) {
        display.clear();
        self.pc += 2;
    }

    fn ret(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    fn jump(&mut self, addr: u16) {
        self.pc = addr;
    }

    fn call(&mut self, addr: u16) {
        self.stack[self.sp as usize] = self.pc + 2;
        self.sp += 1;
        self.pc = addr;
    }

    // if c && a == b, skip or
    // if !c && a != b, skip, otherwise don't skip
    fn skip(&mut self, a: u8, b: u8, c: bool) {
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

    // The interpreter reads n bytes from memory, 
    // starting at the address stored in I. These bytes are 
    // then displayed as sprites on screen at coordinates (Vx, Vy). 
    // Sprites are XORed onto the existing screen. If this causes any
    // pixels to be erased, VF is set to 1, otherwise it is set to 0. 
    // If the sprite is positioned so part of it is outside the coordinates
    // of the display, it wraps around to the opposite side of the screen. 
    // See instruction 8xy3 for more information on XOR, and section 2.4, 
    // Display, for more information on the Chip-8 screen and sprites.

    fn display_sprite(&mut self, ram: &mut Ram, display: &mut Display, x: u8, y: u8, n: u8) {
        self.regs[0xF] = 0;
        for byte in 0..n {
            let y = (self.regs[y as usize] as usize + byte as usize) % CHIP8_HEIGHT;
            for bit in 0..8 {
                let x = (self.regs[x as usize] as usize + bit) % CHIP8_WIDTH;
                let new_pixel = (ram.read(self.i + byte as u16) >> (7 - bit)) & 1;

                let curr = display.read_pixel(x, y);
                self.regs[0xF] |= new_pixel & curr;
                display.write_pixel(x, y, curr ^ new_pixel);
            }
        }
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
    if a > b {
        return (((a - b) & 0xFF) as u8, 1);
    } else {
        return ((a - b) as u8, 0);
    }
}
