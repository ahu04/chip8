# chip-8 emulator

> CHIP-8 is an interpreted programming language, developed by Joseph Weisbecker. It was initially used on the COSMAC VIP and Telmac 1800 8-bit microcomputers in the mid-1970s. CHIP-8 programs are run on a CHIP-8 virtual machine. 

> — [Wikipedia on CHIP-8](https://en.wikipedia.org/wiki/CHIP-8)

Chip-8 allowed video games to be more easily programmable, and portable on early PCs. Its simplicity and popularity means that there are emulators and ROMs being written even today, and a plethora of resources on how to implement an emulator myself.

I wanted to learn Rust, and chip8 is a classic systems project, so here I am! 

Overall, definitely fought with the Rust compiler a bit, learned some SDL2, and some good debugging skills :)

### To build / run on your machine: 

Git clone this repo, then, from base directory (chip8), run "cargo run --release -- pathToROM". For example, if I wanted to run WIPEOFF, I would run "cargo run --release -- roms/WIPEOFF". 

### Resources Used:
- [Cowgod's Chip-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#1.0)
- [Varun's Chip-8 implementation](https://github.com/varunshenoy/rusty-chip8/tree/main)
- [Colin Eberhardt's WASM Chip-8](https://colineberhardt.github.io/wasm-rust-chip8/web/)
