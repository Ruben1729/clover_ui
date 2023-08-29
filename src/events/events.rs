use crate::state::{Key, MouseButton};

#[derive(Debug, Clone, Copy)]
pub enum Events {
    KeyDown(Key),
    KeyUp(Key),

    MouseScroll(Option<(f32, f32)>),

    MouseEnter,
    MouseLeave,
    MouseMove(Option<(f32, f32)>),

    Click(MouseButton),
    MouseDown(MouseButton),
    MouseUp(MouseButton),
}
