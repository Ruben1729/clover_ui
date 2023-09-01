use crate::element::Element;
use crate::element::ElementType::Label;
use crate::paint::Drawable;
use crate::ui::Ui;

impl Ui {
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
}
