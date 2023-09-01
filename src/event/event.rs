use crate::state::{Key, MouseButton, MouseState};

#[derive(Debug, Clone)]
pub enum Event {
    KeyDown(Key),
    KeyUp(Key),

    MouseScroll(MouseState),
    MouseMove(MouseState),

    MouseDown {
        state: MouseState,
        button: MouseButton,
    },
    MouseUp {
        state: MouseState,
        button: MouseButton,
    },
}
