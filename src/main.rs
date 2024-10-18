use macroquad::prelude::*;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

// draw pixel x, y with color
fn draw_pixel(x: i32, y: i32, color: Color) {
    let size_width = screen_width() / WIDTH as f32;
    let size_height = screen_height() / HEIGHT as f32;
    let x = x as f32;
    let y = y as f32;
    let color = color;

    draw_rectangle(x * size_width, y * size_height, size_width, size_height, color);
}

#[macroquad::main("Chip8")]
async fn main() {
    loop {
        clear_background(BLACK);
        

        draw_pixel(0, 0, WHITE);
        draw_pixel(32, 0, WHITE);
        draw_pixel(63, 0, WHITE);

        next_frame().await
    }
}
