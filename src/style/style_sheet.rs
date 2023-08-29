use crate::style::{Color, StyleProperty};

#[derive(Default)]
pub struct StyleSheet(pub(crate) Vec<StyleProperty>);

impl StyleSheet {
    pub fn set_background_color(&mut self, a: u8, r: u8, g: u8, b: u8) {
        self.0.push(StyleProperty::BackgroundColor(Color::new(a, r, g, b)));
    }
}
