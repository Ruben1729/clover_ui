use crate::style::FontWeight;
use rusttype::Scale;

#[derive(Debug, Clone)]
pub enum Primitive {
    Circle {
        x: f32,
        y: f32,
        radius: f32,
        color: u32,
    },
    Rectangle {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: u32,
    },
    Text {
        x: f32,
        y: f32,
        font_size: f32,
        font_weight: FontWeight,
        content: String,
        color: u32,
    },
}
