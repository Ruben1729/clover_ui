use crate::element::{Element, ElementState, ElementType};
use crate::element::ElementType::Label;
use crate::paint::Drawable;
use crate::style::{COLOR_SLATE_50, COLOR_SLATE_600, FlexDirection, Layout, Spacing, StyleSheet};
use crate::ui::Ui;

impl Ui {
    pub fn flex_col(&mut self, add_contents: impl FnOnce(&mut Ui)) {
        // Create node
        let mut new_element = Element::new(ElementType::Layout);

        let element_uuid = new_element.uuid().clone();
        self.style_element(&mut new_element);

        new_element.style.set_backgroundcolor(COLOR_SLATE_50);
        new_element.style.set_width(300);
        new_element.style.set_height(200);

        self.set_cursor_this(&mut new_element);

        // handle events
        self.running_counter += 1;

        if self.context.event_queue.len() > 0 {
            for event in &self.context.event_queue {
                new_element.handle_event(event);
            }

            self.persistent_element_state.insert(self.running_counter, new_element.state_manager.clone());
        } else if let Some(state_manager) = self.persistent_element_state.get(&self.running_counter) {
            new_element.state_manager = state_manager.clone();
        }

        let mut test = StyleSheet::new();
        test.set_backgroundcolor(COLOR_SLATE_600);

        new_element.state_style.insert(ElementState::Hovered, test);

        self.draw_calls.extend(new_element.draw());

        self.parent_layout.push_front(Layout::Flex {
            flex_direction: FlexDirection::Col
        });
        self.set_cursor_inside(&new_element);
        add_contents(self);
        self.parent_layout.pop_front();

        self.set_cursor_next(&new_element);

        // Clear styles and parent stack
        self.unbind_styles(element_uuid);
    }

    pub fn label(&mut self, value: &str) {
        let mut new_element = Element::new(Label(value));

        let element_uuid = new_element.uuid().clone();

        self.style_element(&mut new_element);
        self.compute_text_dimensions(&mut new_element);
        self.set_cursor_this(&mut new_element);

        self.draw_calls.extend(new_element.draw());

        self.set_cursor_next(&new_element);
        self.unbind_styles(element_uuid);
    }
}
