use crate::element::{Element, ElementState, ElementType};
use crate::ui::Ui;
use std::cell::RefCell;
use std::rc::Rc;
use crate::style::{COLOR_BLUE_500, COLOR_BLUE_600, COLOR_WHITE, Spacing, StyleSheet};

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

    pub fn button(&mut self, add_contents: impl FnOnce(&mut Ui)) -> Rc<RefCell<Element>> {
        let mut button_base_styles = StyleSheet::new();
        button_base_styles.set_backgroundcolor(COLOR_BLUE_600);
        button_base_styles.set_color(COLOR_WHITE);
        button_base_styles.set_padding(Spacing::new(5, 10, 5, 10));

        let container = self.with_style_sheet(button_base_styles).flex(add_contents);

        let mut button_styles = StyleSheet::new();
        button_styles.set_backgroundcolor(COLOR_BLUE_500);
        container.borrow_mut().state_style.insert(ElementState::Hovered, button_styles);

        container
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

    pub fn p(&mut self, val: &str) -> Rc<RefCell<Element>> {
        let elem_ref = self.label(val);
        elem_ref.borrow_mut().style.set_fontsize(15.0);
        elem_ref
    }

    pub fn h1(&mut self, val: &str) -> Rc<RefCell<Element>> {
        let elem_ref = self.label(val);
        elem_ref.borrow_mut().style.set_fontsize(30.0);
        elem_ref
    }

    pub fn h2(&mut self, val: &str) -> Rc<RefCell<Element>> {
        let elem_ref = self.label(val);
        elem_ref.borrow_mut().style.set_fontsize(20.0);
        elem_ref
    }
}
