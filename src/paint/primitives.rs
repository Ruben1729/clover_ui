use rusttype::{Font, Scale};
use crate::style::Color;

pub enum Primitive {
    Circle {
        x:              f32,
        y:              f32,
        radius:         f32,
        color:          u32
    },
    Rectangle {
        x:              f32,
        y:              f32,
        width:          f32,
        height:         f32,
        color:          u32
    },
    Text {
        x:              f32,
        y:              f32,
        scale:          Scale,
        content:        String,
        color:          u32
    }
}
