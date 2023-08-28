use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::element::{Element, ElementState};
use crate::layout::{Color, Layout};

#[derive(Default)]
pub struct ElementBuilder {
    id:         String,
    class:      Vec<String>,
    layout:     Layout,
    conditional_layouts: HashMap<ElementState, Color>
}

impl ElementBuilder {
    pub fn with_id(mut self, id: String) -> Self {
        self.id = id;
        self
    }

    pub fn with_class(mut self, class: String) -> Self {
        self.class.push(class);
        self
    }

    pub fn with_layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    pub fn with_layout_on_hover(mut self, color: Color) -> Self {
        self.conditional_layouts.insert(ElementState::Hovered, color);
        self
    }

    pub fn build(self) -> Rc<RefCell<Element>>{
        Rc::new(RefCell::new(Element::new(
            self.id,
            self.class,
            self.layout,
            self.conditional_layouts
        )))
    }
}
