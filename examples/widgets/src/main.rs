extern crate minifb;

use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn draw_rectangle(buffer: &mut [u32], x: usize, y: usize, width: usize, height: usize, color: u32) {
    for iy in 0..height {
        for ix in 0..width {
            let x_pos = x + ix;
            let y_pos = y + iy;
            if x_pos < WIDTH && y_pos < HEIGHT {
                let index = y_pos * WIDTH + x_pos;
                buffer[index] = color;
            }
        }
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale_mode: ScaleMode::UpperLeft,
            ..Default::default()
        },
    )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Draw a blue rectangle at (50, 50) with dimensions (100, 100)
        draw_rectangle(&mut buffer, 50, 50, 100, 100, 0xFF_00_00);

        // Draw a red rectangle at (200, 200) with dimensions (150, 100)
        draw_rectangle(&mut buffer, 200, 200, 150, 100, 0x00_00_FF);

        // Update the window with the buffer data
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}