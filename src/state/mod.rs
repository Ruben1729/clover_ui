mod keys;
mod mouse;

pub use self::keys::*;
pub use self::mouse::*;

#[derive(Default)]
pub struct State {
    pub(crate) keys: KeyState,
    pub(crate) mouse: MouseState,
}

impl State {
    pub fn clear(&mut self) {
        self.keys.clear();
        self.mouse.clear();
    }
}
