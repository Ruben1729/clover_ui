use crate::element::ElementType::Label;
use crate::element::{Element, ElementState, ElementType};
use crate::paint::Drawable;
use crate::style::{
    FlexDirection, FontWeight, Layout, Spacing, StyleSheet, COLOR_BLUE_500, COLOR_BLUE_600,
    COLOR_SLATE_50,
};
use crate::ui::Ui;

impl Ui {
    pub fn create_widget(&mut self, new_element: &mut Element) {}

    pub fn button(&mut self, add_contents: impl FnOnce(&mut Ui)) -> &Self {
        let mut new_element = Element::new(ElementType::Container(Layout {
            flex_direction: FlexDirection::Row,
        }));

        let mut button_style = StyleSheet::new();
        button_style.set_backgroundcolor(COLOR_BLUE_600);
        button_style.set_color(COLOR_SLATE_50);
        button_style.set_padding(Spacing::new(5, 10, 5, 10));
        button_style.set_width(100);
        button_style.set_height(20);

        let mut hover_style = StyleSheet::new();
        hover_style.set_backgroundcolor(COLOR_BLUE_500);

        self.with_style_sheet(&button_style);
        new_element
            .state_style
            .insert(ElementState::Hovered, hover_style);

        self.create_layout(&mut new_element, add_contents);
        self.unbind_styles(new_element.uuid());
        self
    }
    pub fn label(&mut self, value: &str) {
        let mut new_element = Element::new(Label(value));

        let element_uuid = new_element.uuid().clone();

        self.inherit_style(&mut new_element);
        self.compute_text_dimensions(&mut new_element);
        self.move_to_cursor(&mut new_element);

        self.draw_calls.extend(new_element.draw());

        self.move_cursor_to_next_element(&new_element);
        self.unbind_styles(element_uuid);
    }

    pub fn h1(&mut self, value: &str) {
        let mut new_element = Element::new(Label(value));

        let element_uuid = new_element.uuid().clone();

        self.inherit_style(&mut new_element);
        self.compute_text_dimensions(&mut new_element);
        self.move_to_cursor(&mut new_element);

        new_element.style.set_fontweight(FontWeight::Bold);

        self.draw_calls.extend(new_element.draw());

        self.move_cursor_to_next_element(&new_element);
        self.unbind_styles(element_uuid);
    }
}
