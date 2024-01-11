use glyphon::{FamilyOwned, Weight};
use crate::styles::Color;

#[derive(Debug, Clone)]
pub enum Primitive {
    Circle {
        x: f32,
        y: f32,
        z: f32,
        radius: f32,
        color: Color,
    },
    Rectangle {
        x: f32,
        y: f32,
        z: f32,
        width: f32,
        height: f32,
        color: Color,
    },
    Text {
        x: f32,
        y: f32,
        z: f32,
        value: String,
        width: f32,
        height: f32,
        family: FamilyOwned,
        font_size: f32,
        weight: Weight,
        line_height: f32,
        color: Color
    }
}