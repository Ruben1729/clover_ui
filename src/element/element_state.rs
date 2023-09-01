#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum ElementState {
    Hovered,
    Focused,
    Clicked
}

#[derive(Default, Clone)]
pub struct ElementStateManager {
    pub states: Vec<ElementState>
}

impl ElementStateManager {
    pub fn push(&mut self, state: ElementState) {
        self.remove(state.clone());
        self.states.push(state);
    }

    pub fn pop(&mut self) -> Option<ElementState> {
        self.states.pop()
    }

    pub fn peek(&self) -> Option<&ElementState> {
        self.states.last()
    }

    pub fn clear(&mut self) {
        self.states.clear();
    }

    pub fn remove(&mut self, state: ElementState) {
        self.states.retain(|s| *s != state);
    }

    pub fn is(&self, state: ElementState) -> bool {
        self.states.contains(&state)
    }
}
