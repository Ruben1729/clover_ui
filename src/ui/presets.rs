use crate::element::{Element, ElementState, ElementType};
use crate::style::{Spacing, StyleSheet, COLOR_BLUE_500, COLOR_BLUE_600, COLOR_WHITE};
use crate::ui::Ui;

impl<'a> Ui<'a> {
    pub fn flex(&mut self, add_contents: impl FnOnce(&mut Ui)) -> &mut Element<'a> {
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
        &mut self.dom_tree[idx]
    }

    pub fn button(&mut self, add_contents: impl FnOnce(&mut Ui)) -> &mut Element<'a> {
        let mut button_base_styles = StyleSheet::new();
        button_base_styles.set_backgroundcolor(COLOR_BLUE_600);
        button_base_styles.set_color(COLOR_WHITE);
        button_base_styles.set_padding(Spacing::new(5, 10, 5, 10));

        let elem = self.with_style_sheet(button_base_styles).flex(add_contents);

        let mut button_styles = StyleSheet::new();
        button_styles.set_backgroundcolor(COLOR_BLUE_500);
        elem.state_style.insert(ElementState::Hovered, button_styles);
        elem
    }

    pub fn label(&mut self, val: &str) -> &mut Element<'a> {
        let mut new_element = Element::new(
            ElementType::Label(val.to_string()),
            Some(self.current_parent.front().unwrap().clone()),
        );

        let element_uuid = new_element.uuid().clone();

        // Create node and compute dimensions
        self.style_element(&mut new_element);
        let idx = self.add_element(new_element);
        self.unbind_styles(element_uuid);
        &mut self.dom_tree[idx]
    }
}
