use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::element::{Element, ElementState};
use crate::style::{ConditionalStyle, Style};

#[derive(Default)]
pub struct ElementBuilder {
    id:         String,
    class:      Vec<String>,
    style: Style,
    conditional_styles: HashMap<ElementState, ConditionalStyle>
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

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn with_style_on_hover(mut self, styles: ConditionalStyle) -> Self {
        self.conditional_styles.insert(ElementState::Hovered, styles);
        self
    }

    pub fn build(self) -> Rc<RefCell<Element>>{
        Rc::new(RefCell::new(Element::new(
            self.id,
            self.class,
            self.style,
            self.conditional_styles
        )))
    }
}
