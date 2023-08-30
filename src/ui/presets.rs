use std::cell::RefCell;
use std::rc::Rc;
use crate::element::{Element, ElementType};
use crate::ui::Ui;

impl Ui {
    pub fn flex(&mut self, add_contents: impl FnOnce(&mut Ui)) -> Rc<RefCell<Element>> {
        // Create node
        let new_node = Element::new(
            "flex".to_string(),
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

    pub fn button(&mut self, id: String) -> Rc<RefCell<Element>> {
        let new_node = Element::new(
            id,
            ElementType::Button,
            Some(self.current_parent.front().unwrap().clone()),
        );

        self.create_node(new_node.clone());
        self.unbind_styles(new_node.clone());
        new_node
    }

    pub fn label(&mut self, id: String) -> Rc<RefCell<Element>> {
        let new_node = Element::new(
            id,
            ElementType::Label,
            Some(self.current_parent.front().unwrap().clone()),
        );

        self.create_node(new_node.clone());
        self.unbind_styles(new_node.clone());
        new_node
    }
}