use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::element::{Element, ElementState, ElementType};
use crate::style::{ConditionalStyle, StyleBuilder};

#[derive(Default)]
pub struct ElementBuilder {
    ty:                 ElementType,
    id:                 String,
    class:              Vec<String>,
    styles:             ConditionalStyle,
    conditional_styles: HashMap<ElementState, ConditionalStyle>
}

impl ElementBuilder {
    pub fn new(ty: ElementType) -> Self {
        ElementBuilder {
            ty,
            id: Default::default(),
            class: Default::default(),
            styles: Default::default(),
            conditional_styles: Default::default()
        }
    }
    pub fn with_id(mut self, id: String) -> Self {
        self.id = id;
        self
    }

    pub fn with_class(mut self, class: String) -> Self {
        self.class.push(class);
        self
    }

    pub fn with_styles(mut self, styles: ConditionalStyle) -> Self {
        self.styles = styles;
        self
    }

    pub fn with_style_on_hover(mut self, styles: ConditionalStyle) -> Self {
        self.conditional_styles.insert(ElementState::Hovered, styles);
        self
    }

    pub fn build(self) -> Rc<RefCell<Element>>{
        Rc::new(RefCell::new(Element::new(
            self.ty,
            self.id,
            self.class,
            StyleBuilder::from(self.styles),
            self.conditional_styles
        )))
    }
}
