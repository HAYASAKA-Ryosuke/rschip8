use macroquad::prelude::*;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    pub pixels: [[u8; HEIGHT]; WIDTH],
    pub width: usize,
    pub height: usize
}

impl Display {
    pub fn new () -> Display {
        Display {
            pixels: [[0; HEIGHT]; WIDTH],
            width: WIDTH,
            height: HEIGHT
        }
    }
    pub fn clear(&mut self) {
        self.pixels = [[0; HEIGHT]; WIDTH];
    }
    pub fn update(&mut self) {
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
