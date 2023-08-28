use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use crate::events::Events;
use crate::style::{Color, ConditionalStyle, Style, StyleProperty};

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
    pub style: Style,
    conditional_styles:    HashMap<ElementState, ConditionalStyle>,

    parent:                 Option<Rc<RefCell<Element>>>,
    pub children:           Vec<Rc<RefCell<Element>>>,

    pub state:              HashSet<ElementState>
}

impl Element {
    pub fn new(id: String, class: Vec<String>, layout: Style, conditional_styles: HashMap<ElementState, ConditionalStyle>) -> Self {
        Element {
            ty:         ElementType::Div,
            id,
            class,
            style: layout,
            conditional_styles,

            parent:     None,
            children:   Vec::new(),

            state:      HashSet::new()
        }
    }

    pub fn insert(parent: &Rc<RefCell<Self>>, child: &Rc<RefCell<Self>>) {
        child.borrow_mut().parent = Some(Rc::clone(parent));
        parent.borrow_mut().children.push(Rc::clone(child));
    }

    pub fn style(&self) -> Style {
        let mut style = self.style.clone();

        for state in &self.state {
            if let Some(styles) = self.conditional_styles.get(state) {
                for curr_style in styles {
                    match curr_style {
                        StyleProperty::Padding(val) => {
                            style.padding = val.clone();
                        }
                        StyleProperty::Margin(val) => {
                            style.margin = val.clone();
                        }
                        StyleProperty::Border(val) => {
                            style.border = val.clone();
                        }
                        StyleProperty::Height(val) => {
                            style.height = val.clone();
                        }
                        StyleProperty::Width(val) => {
                            style.width = val.clone();
                        }
                        StyleProperty::X(val) => {
                            style.x = val.clone();
                        }
                        StyleProperty::Y(val) => {
                            style.y = val.clone();
                        }
                        StyleProperty::Display(val) => {
                            style.display = val.clone();
                        }
                        StyleProperty::BackgroundColor(val) => {
                            style.background_color = val.clone();
                        }
                        StyleProperty::Color(val) => {
                            style.color = val.clone();
                        }
                    }
                }
            }
        }

        style
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
                    if position.0 > self.style.x as f32 && position.0 < (self.style.x + self.style.width()) as f32 &&
                       position.1 > self.style.y as f32 && position.1 < (self.style.y + self.style.height()) as f32 {
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
