use crate::element::{Element, ElementType};
use crate::ui::Ui;

impl<'a> Ui<'a> {
    pub fn flex(&mut self, add_contents: impl FnOnce(&mut Ui)) {
        // Create node
        let mut new_element = Element::new(
            ElementType::Layout,
            Some(self.current_parent.front().unwrap().clone()),
        );

        let element_uuid = new_element.uuid().clone();

        self.style_element(&mut new_element);
        let idx = self.add_element(new_element);

        // Set current parent to this node and add content
        self.current_parent.push_front(idx);
        add_contents(self);

        // Clear styles and parent stack
        self.unbind_styles(element_uuid);
        self.current_parent.pop_front();
    }

    pub fn label(&mut self, val: &str) {
        let mut new_element = Element::new(
            ElementType::Label(val.to_string()),
            Some(self.current_parent.front().unwrap().clone()),
        );

        let element_uuid = new_element.uuid().clone();

        // Create node and compute dimensions
        self.style_element(&mut new_element);
        self.add_element(new_element);
        self.unbind_styles(element_uuid);
    }
}
