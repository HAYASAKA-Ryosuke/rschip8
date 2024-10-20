use std::env;
use std::fs::File;
use std::io::Read;
use macroquad::prelude::*;
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
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: chip8 <rom>");
        std::process::exit(1);
    }
    let rom = read_rom(&args[1]);
    let display = Display::new();
    let audio = Audio::new("./beep.wav");
    let mut cpu = Cpu::new(display, audio.await);
    cpu.load_rom(rom);
    loop {
        cpu.key_input();
        cpu.emulate_cycle();
        cpu.update_timers();
        cpu.display_update();
        next_frame().await
    }
}
