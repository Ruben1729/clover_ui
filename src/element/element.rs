use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use crate::events::Events;
use crate::layout::{Color, Layout};

pub enum ElementType {
    Div,
    Text {
        value: String,
    },
    CheckBox {
        value: bool,
    },
    Button {
        value: bool,
    },
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ElementState {
    Hovered
}

pub struct Element {
    ty:                     ElementType,
    id:                     String,
    class:                  Vec<String>,
    pub layout:             Layout,
    conditional_layouts:    HashMap<ElementState, Color>,

    parent:                 Option<Rc<RefCell<Element>>>,
    pub children:           Vec<Rc<RefCell<Element>>>,

    pub state:              HashSet<ElementState>
}

impl Element {
    pub fn new(id: String, class: Vec<String>, layout: Layout, cond: HashMap<ElementState, Color>) -> Self {
        Element {
            ty:         ElementType::Div,
            id,
            class,
            layout,
            conditional_layouts: cond,

            parent:     None,
            children:   Vec::new(),

            state:      HashSet::new()
        }
    }

    pub fn insert(parent: &Rc<RefCell<Self>>, child: &Rc<RefCell<Self>>) {
        child.borrow_mut().parent = Some(Rc::clone(parent));
        parent.borrow_mut().children.push(Rc::clone(child));
    }

    pub fn layout(&self) -> Layout {
        let mut layout = self.layout.clone();

        // TODO: this is extremely inefficient
        for state in self.state.iter() {
            if let Some(color) = self.conditional_layouts.get(state) {
                layout.border.set_color(*color);
            }
        }

        layout
    }

    pub fn is_hovered(&self) -> bool {
        self.state.contains(&ElementState::Hovered)
    }

    pub fn handle_event(&mut self, event: &Events) {
        match event {
            Events::KeyDown(_) => {}
            Events::KeyUp(_) => {}
            Events::MouseScroll(_) => {}
            Events::MouseEnter => {}
            Events::MouseLeave => {}
            Events::MouseMove(move_event) => {
                if let Some(position) = move_event {
                    if position.0 > self.layout.x as f32 && position.0 < (self.layout.x + self.layout.width()) as f32 &&
                       position.1 > self.layout.y as f32 && position.1 < (self.layout.y + self.layout.height()) as f32 {
                        self.state.insert(ElementState::Hovered);
                    } else {
                        self.state.remove(&ElementState::Hovered);
                    }
                } else {
                    self.state.remove(&ElementState::Hovered);
                }
            }
            Events::Click(_) => {}
            Events::MouseDown(_) => {}
            Events::MouseUp(_) => {}
        }
    }
}
