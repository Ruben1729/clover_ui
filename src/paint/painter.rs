use crate::paint::Primitive;
use crate::style::{Color, FontManager, FontWeight};
use rusttype::{point, Font};

// Define a trait for the drawing backend
pub trait DrawingBackend {
    fn draw_circle(&mut self, x: f32, y: f32, radius: f32, color: u32);
    fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32, color: u32);
    fn draw_pixel(&mut self, x: usize, y: usize, color: u32);
}

pub struct Painter<'a> {
    draw_calls: Vec<Primitive>,
    backend: &'a mut dyn DrawingBackend,
}

impl<'a> Painter<'a> {
    pub fn new(backend: &'a mut dyn DrawingBackend) -> Self {
        Painter {
            draw_calls: Vec::new(),
            backend,
        }
    }

    pub fn lerp_color(back: u32, front: u32) -> u32 {
        let alpha = ((front >> 24) & 0xFF) as f32 / 255.0;

        let r1 = ((back >> 16) & 0xFF) as f32;
        let g1 = ((back >> 8) & 0xFF) as f32;
        let b1 = (back & 0xFF) as f32;

        let r2 = ((front >> 16) & 0xFF) as f32;
        let g2 = ((front >> 8) & 0xFF) as f32;
        let b2 = (front & 0xFF) as f32;

        let r = (r1 * (1.0 - alpha) + r2 * alpha) as u32;
        let g = (g1 * (1.0 - alpha) + g2 * alpha) as u32;
        let b = (b1 * (1.0 - alpha) + b2 * alpha) as u32;

        (0xFF << 24) | (r << 16) | (g << 8) | b
    }

    pub fn extend(&mut self, calls: Vec<Primitive>) {
        self.draw_calls.extend(calls);
    }
    pub fn draw(&mut self) {
        for call in &self.draw_calls {
            match call {
                Primitive::Circle {
                    x,
                    y,
                    radius,
                    color,
                } => self.backend.draw_circle(*x, *y, *radius, *color),
                Primitive::Rectangle {
                    x,
                    y,
                    width,
                    height,
                    color,
                } => self.backend.draw_rect(*x, *y, *width, *height, *color),
                Primitive::Text {
                    x,
                    y,
                    scale,
                    content,
                    color,
                } => {
                    let manager = FontManager::get();
                    let font = manager
                        .get_font(None, FontWeight::Bold)
                        .expect("Unable to load font");

                    let v_metrics = font.v_metrics(*scale);

                    // layout the glyphs in a line with 20 pixels padding
                    let glyphs: Vec<_> = font
                        .layout(content, *scale, point(*x, *y + v_metrics.ascent))
                        .collect();

                    // Loop through the glyphs in the text, positing each one on a line
                    for glyph in glyphs {
                        if let Some(bounding_box) = glyph.pixel_bounding_box() {
                            // Draw the glyph into the image per-pixel by using the draw closure
                            glyph.draw(|x_, y_, v| {
                                let new_x = (x_ + bounding_box.min.x as u32) as usize;
                                let new_y = (y_ + bounding_box.min.y as u32) as usize;
                                let mut new_color = Color::new_u32(*color);
                                new_color.set_alpha((v * 255.0) as u8);
                                self.backend.draw_pixel(new_x, new_y, new_color.get_u32());
                            });
                        }
                    }
                }
            }
        }
    }
}
