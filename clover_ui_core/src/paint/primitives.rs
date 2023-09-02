use bytemuck::{Pod, Zeroable};
use crate::style::FontWeight;
use rusttype::Scale;

#[cfg(feature = "primitive_vertex")]
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Primitive {
    pub position: [f32; 2],
    pub color: u32,
}

#[cfg(feature = "primitive_shapes")]
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
