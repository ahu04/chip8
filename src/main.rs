use std::{env, process, fs::File, io::Read};

mod chip8;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!{"Usage: ./chip8 rom_file_name"}
        process::exit(1);
    }
    let mut rom_file = read_file_or_fail(&args[1]);
    let mut rom_data = Vec::<u8>::new();
    rom_file.read_to_end(&mut rom_data).expect("failed to read file data");
    let mut chip8 = chip8::Chip8::new();
    chip8.init_rom(&rom_data);
    chip8.run();
}

fn read_file_or_fail(filename : &String) -> File {
    match File::open(filename) {
        Ok(file) => return file,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    }
}

