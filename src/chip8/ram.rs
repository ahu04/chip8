const RAM_SIZE: usize = 4096;
const RAM_START: usize = 0x200;

pub struct Ram {
    data: [u8; RAM_SIZE],
}

impl Ram {
    pub fn new() -> Ram {
        let mut ram = Ram {
            data: [0; RAM_SIZE],
        };

        let digit_sprites: [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0],
            [0x20, 0x60, 0x20, 0x20, 0x70],
            [0xF0, 0x10, 0xF0, 0x80, 0xF0],
            [0xF0, 0x10, 0xF0, 0x10, 0xF0],
            [0x90, 0x90, 0xF0, 0x10, 0x10],
            [0xF0, 0x80, 0xF0, 0x10, 0xF0],
            [0xF0, 0x80, 0xF0, 0x90, 0xF0],
            [0xF0, 0x10, 0x20, 0x40, 0x40],
            [0xF0, 0x90, 0xF0, 0x90, 0xF0],
            [0xF0, 0x90, 0xF0, 0x10, 0xF0],
            [0xF0, 0x90, 0xF0, 0x90, 0x90],
            [0xE0, 0x90, 0xE0, 0x90, 0xE0],
            [0xF0, 0x80, 0x80, 0x80, 0xF0],
            [0xE0, 0x90, 0x90, 0x90, 0xE0],
            [0xF0, 0x80, 0xF0, 0x80, 0xF0],
            [0xF0, 0x80, 0xF0, 0x80, 0x80],
        ];

        let mut i = 0;
        for arr in digit_sprites {
            for byte in arr {
                ram.data[i] = byte;
                i += 1;
            }
        }
        return ram;
    }

    pub fn init_rom(&mut self, rom: &Vec<u8>) {
        // byte by byte initialize ramory, if it's valid
        if rom.len() > RAM_SIZE - RAM_START {
            panic!("invalid rom");
        }
        for i in 0..rom.len() {
            self.data[i + RAM_START] = rom[i];
        }
    }

    // read one byte
    pub fn read(&mut self, i: u16) -> u8 {
        return self.data[i as usize];
    }

    pub fn write(&mut self, i: u16, b: u8) {
        if i < RAM_START as u16 {
            panic!("tried to write to protected mem, addr: {:x}", i);
        }
        self.data[i as usize] = b;
    }
}
