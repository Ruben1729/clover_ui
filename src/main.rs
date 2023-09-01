use clover_ui::context::Context;
use clover_ui::paint::{DrawingBackend, Painter};
use clover_ui::style::{Color, FontManager, Spacing, StyleSheet};
use clover_ui::ui::Ui;
use minifb::{Key, MouseMode, ScaleMode, Window, WindowOptions};
use std::path::Path;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

pub struct MiniFbBackend {
    buffer: Vec<u32>,
}

impl MiniFbBackend {
    pub fn new() -> Self {
        Self {
            buffer: vec![0; WIDTH * HEIGHT],
        }
    }
}

impl DrawingBackend for MiniFbBackend {
    fn draw_circle(&mut self, x: f32, y: f32, radius: f32, color: u32) {
        let mut x_coord = radius as i32;
        let mut y_coord = 0i32;
        let mut p = 1 - x_coord as i32;

        self.draw_symmetric_points(x as i32, y as i32, x_coord, y_coord, color);

        while x_coord > y_coord {
            y_coord += 1;

            if p < 0 {
                p += 2 * y_coord + 1;
            } else {
                x_coord -= 1;
                p += 2 * (y_coord - x_coord) + 1;
            }

            self.draw_symmetric_points(x as i32, y as i32, x_coord, y_coord, color);
        }
    }

    fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32, color: u32) {
        for i in 0..(width as usize) {
            for j in 0..(height as usize) {
                let x_pixel = x as usize + i;
                let y_pixel = y as usize + j;

                if x_pixel < WIDTH && y_pixel < HEIGHT {
                    self.draw_pixel(x_pixel, y_pixel, color);
                }
            }
        }
    }

    fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < WIDTH && y < HEIGHT {
            let old_color = self.buffer[y * WIDTH + x];
            let new_color = Painter::lerp_color(old_color, color);
            self.buffer[y * WIDTH + x] = new_color;
        }
    }
}

impl MiniFbBackend {
    pub fn clear(&mut self) {
        unsafe {
            std::ptr::write_bytes(self.buffer.as_mut_ptr(), 0, self.buffer.len());
        }
    }
    fn draw_symmetric_points(&mut self, xc: i32, yc: i32, x: i32, y: i32, color: u32) {
        self.draw_pixel((xc + x) as usize, (yc + y) as usize, color);
        self.draw_pixel((xc - x) as usize, (yc + y) as usize, color);
        self.draw_pixel((xc + x) as usize, (yc - y) as usize, color);
        self.draw_pixel((xc - x) as usize, (yc - y) as usize, color);
        self.draw_pixel((xc + y) as usize, (yc + x) as usize, color);
        self.draw_pixel((xc - y) as usize, (yc + x) as usize, color);
        self.draw_pixel((xc + y) as usize, (yc - x) as usize, color);
        self.draw_pixel((xc - y) as usize, (yc - x) as usize, color);
    }
}

fn main() {
    {
        let mut font_manager = FontManager::get_mut();
        font_manager
            .load(
                None,
                Path::new("/run/media/rubens/ssd/projects/clover_ui/inter.ttf"),
            )
            .expect("Unable to load font");
    }

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

    let mut backend = MiniFbBackend::new();
    let mut ui = Ui::default();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        backend.clear();

        if window.get_mouse_down(minifb::MouseButton::Left) {
            ui.context.set_mouse_pressed(clover_ui::state::MouseButton::Left);
        }
        if window.get_mouse_down(minifb::MouseButton::Right) {
            ui.context.set_mouse_pressed(clover_ui::state::MouseButton::Right);
        }
        if window.get_mouse_down(minifb::MouseButton::Middle) {
            ui.context.set_mouse_pressed(clover_ui::state::MouseButton::Middle);
        }

        ui.context.set_mouse_pos(window.get_unscaled_mouse_pos(MouseMode::Discard));
        ui.context.set_mouse_scroll(window.get_scroll_wheel());

        // Updates the state and generates events
        ui.context.next();

        let mut text_ss = StyleSheet::new();
        text_ss.set_color(Color::new(255, 20, 20, 20));
        text_ss.set_fontsize(15.4);

        ui.with_style_sheet(text_ss).flex_col(|ui| {
            ui.flex_col(|ui| {
                ui.label("Save");
                ui.label("Cancel");
            });
        });

        {
            let mut painter = Painter::new(&mut backend);
            painter.extend(ui.draw_calls.clone());
            painter.draw();
        }

        ui.clear();

        window
            .update_with_buffer(&backend.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
