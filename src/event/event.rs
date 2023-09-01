use crate::state::{Key, MouseButton};

#[derive(Debug, Clone, Copy)]
pub enum Event {
    KeyDown(Key),
    KeyUp(Key),

    MouseScroll(Option<(f32, f32)>),
    MouseMove(Option<(f32, f32)>),

    Click(MouseButton),
    MouseDown(MouseButton),
    MouseUp(MouseButton),
}
