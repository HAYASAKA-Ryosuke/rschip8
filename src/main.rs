use macroquad::prelude::*;
use std::fs::File;
use std::io::Read;
mod cpu;
use cpu::Cpu;
mod display;
use display::Display;


fn read_rom(file_name: &str) -> Vec<u8> {
    let mut file = File::open(file_name).unwrap();
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf).unwrap();
    buf
}

#[macroquad::main("Chip8")]
async fn main() {
    let rom = read_rom("./test_opcode.ch8");
    let display = Display::new();
    let mut cpu = Cpu::new(display);
    cpu.load_rom(rom);
    loop {
        cpu.emulate_cycle();
        cpu.display_update();
        cpu.update_timers();
        next_frame().await
    }
}
