#[derive(Default)]
pub struct MouseState {
    pub pressed: Vec<MouseButton>,
    pub pos: Option<(f32, f32)>,
    pub scroll_wheel: Option<(f32, f32)>,
}

impl MouseState {
    pub fn clear(&mut self) {
        self.pressed.clear();
        self.pos = None;
        self.scroll_wheel = None;
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}
