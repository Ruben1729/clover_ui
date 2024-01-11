
#[derive(Debug, Clone, Copy)]
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
}