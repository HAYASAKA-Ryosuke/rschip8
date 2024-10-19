use macroquad::prelude::*;
use std::fs::File;
use std::io::Read;
mod cpu;
use cpu::Cpu;
mod display;
use display::Display;
mod audio;
use audio::Audio;


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
    let audio = Audio::new("./beep.wav");
    let mut cpu = Cpu::new(display, audio.await);
    cpu.load_rom(rom);
    loop {
        cpu.emulate_cycle();
        cpu.display_update();
        cpu.update_timers();
        next_frame().await
    }
}
