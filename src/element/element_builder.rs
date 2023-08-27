use std::cell::RefCell;
use std::rc::Rc;
use crate::element::Element;
use crate::layout::Layout;

#[derive(Default)]
pub struct ElementBuilder {
    id:         String,
    class:      Vec<String>,
    layout:     Layout
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

    pub fn build(self) -> Rc<RefCell<Element>>{
        Rc::new(RefCell::new(Element::new(
            self.id,
            self.class,
            self.layout
        )))
    }
}
