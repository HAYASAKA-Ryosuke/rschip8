use macroquad::prelude::*;
use std::fs::File;
use std::io::Read;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

struct MonoDisplay {
    pub pixels: [[u8; HEIGHT]; WIDTH]
}

trait Display {
    fn clear(&mut self);
    fn update(&mut self);
    fn draw_pixel(&mut self, x: i32, y: i32, color: Color);
}

impl Display for MonoDisplay {
    fn clear(&mut self) {
        self.pixels = [[0; HEIGHT]; WIDTH];
    }
    fn update(&mut self) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let color = if self.pixels[x][y] == 0 { BLACK } else { WHITE };
                self.draw_pixel(x as i32, y as i32, color);
            }
        }
    }
    fn draw_pixel(&mut self, x: i32, y: i32, color: Color) {
        let size_width = screen_width() / WIDTH as f32;
        let size_height = screen_height() / HEIGHT as f32;
        let x = x as f32;
        let y = y as f32;
        let color = color;
        draw_rectangle(x * size_width, y * size_height, size_width, size_height, color);
    }
}

fn read_rom(file_name: &str) -> Vec<u8> {
    let mut file = File::open(file_name).unwrap();
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf).unwrap();
    buf
}

#[macroquad::main("Chip8")]
async fn main() {
    let rom = read_rom("./test_opcode.ch8");
    println!("{:?}", rom);
    let mut mono_display = MonoDisplay {
        pixels: [[0; HEIGHT]; WIDTH]
    };
    mono_display.clear();
    loop {
        mono_display.pixels[1][20] = 1;
        mono_display.update();

        next_frame().await
    }
}
