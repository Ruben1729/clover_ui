#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum ElementState {
    Hovered,
    Focused,
    MouseDown,
    Clicked,
}

#[derive(Default, Clone)]
pub struct ElementStateManager {
    pub states: Vec<ElementState>,
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

    pub fn contains(&self, state: ElementState) -> bool {
        self.states.contains(&state)
    }

    pub fn next(&mut self, state: &ElementState) {

    }

    pub fn is_clicked(&self) -> bool {
        self.contains(ElementState::Clicked)
    }

    pub fn is_hovered(&self) {
        self.contains(ElementState::Hovered);
    }

    pub fn is_focused(&self) {
        self.contains(ElementState::Focused);
    }
}
