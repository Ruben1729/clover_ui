use crate::element::{Element, ElementType};
use crate::ui::Ui;
use std::cell::RefCell;
use std::rc::Rc;

impl Ui {
    pub fn flex(&mut self, add_contents: impl FnOnce(&mut Ui)) -> Rc<RefCell<Element>> {
        // Create node
        let new_node = Element::new(
            ElementType::Layout,
            Some(self.current_parent.front().unwrap().clone()),
        );
        self.create_node(new_node.clone());

        // Set current parent to this node and add content
        self.current_parent.push_front(new_node.clone());
        add_contents(self);

        // Clear styles and parent stack
        self.unbind_styles(new_node.clone());
        self.current_parent.pop_front();
        new_node
    }

    pub fn button(&mut self) -> Rc<RefCell<Element>> {
        let new_node = Element::new(
            ElementType::Button,
            Some(self.current_parent.front().unwrap().clone()),
        );

        // Create node and compute dimensions
        self.create_node(new_node.clone());
        self.unbind_styles(new_node.clone());
        new_node
    }

    pub fn label(&mut self, val: &str) -> Rc<RefCell<Element>> {
        let new_node = Element::new(
            ElementType::Label(val.to_string()),
            Some(self.current_parent.front().unwrap().clone()),
        );

        // Create node and compute dimensions
        self.create_node(new_node.clone());
        self.unbind_styles(new_node.clone());
        new_node
    }
}
