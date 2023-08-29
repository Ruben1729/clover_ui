use crate::paint::Primitive;
use crate::style::Color;
use rusttype::{point, Font, Scale};

// Define a trait for the drawing backend
pub trait DrawingBackend {
    fn draw_circle(&mut self, x: f32, y: f32, radius: f32, color: u32);
    fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32, color: u32);
    fn draw_pixel(&mut self, x: f32, y: f32, color: u32);
    fn draw_pixel_with_blending(&mut self, x: f32, y: f32, color: u32, v: f32);
}

pub struct Painter<'a> {
    draw_calls: Vec<Primitive>,
    backend: &'a mut dyn DrawingBackend,
    font: Font<'static>,
}

impl<'a> Painter<'a> {
    pub fn new(backend: &'a mut dyn DrawingBackend) -> Self {
        let font_data = include_bytes!("/run/media/rubens/ssd/projects/clover_ui/inter.ttf");
        let font = Font::try_from_bytes(font_data as &[u8]).unwrap();
        Painter {
            draw_calls: Vec::new(),
            backend,
            font,
        }
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
                    let v_metrics = self.font.v_metrics(*scale);

                    // layout the glyphs in a line with 20 pixels padding
                    let glyphs: Vec<_> = self
                        .font
                        .layout(content, *scale, point(*x, *y + v_metrics.ascent))
                        .collect();

                    // Loop through the glyphs in the text, positing each one on a line
                    for glyph in glyphs {
                        if let Some(bounding_box) = glyph.pixel_bounding_box() {
                            // Draw the glyph into the image per-pixel by using the draw closure
                            glyph.draw(|x_, y_, v| {
                                let new_x = x_ + bounding_box.min.x as u32;
                                let new_y = y_ + bounding_box.min.y as u32;
                                self.backend.draw_pixel_with_blending(
                                    new_x as f32,
                                    new_y as f32,
                                    *color,
                                    v,
                                );
                            });
                        }
                    }
                }
            }
        }
    }
}
