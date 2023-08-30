use crate::style::{StylePropertyKey, StylePropertyMap};

pub type StyleSheet = StylePropertyMap;

impl StyleSheet {
    pub fn get_total_width(&self) -> usize {
        self.get_margin().horizontal()
            + self.get_padding().horizontal()
            + self.get_borderwidth().horizontal()
            + self.get_padding().horizontal()
            + self.get_width()
    }
    pub fn get_total_height(&self) -> usize {
        self.get_margin().vertical()
            + self.get_padding().vertical()
            + self.get_borderwidth().vertical()
            + self.get_padding().vertical()
            + self.get_width()
    }
    pub fn get_content_y(&self) -> usize {
        self.get_x()
            + self.get_margin().left
            + self.get_borderwidth().left
            + self.get_padding().left
    }

    pub fn get_content_x(&self) -> usize {
        self.get_y() + self.get_margin().top + self.get_borderwidth().top + self.get_padding().top
    }
    pub fn apply(&mut self, other: &StyleSheet) {
        for (key, value) in other.values.iter() {
            self.values.insert(key.clone(), value.clone());
        }
    }

    pub fn inherit(&mut self, other: &StyleSheet) {
        for (key, value) in other.values.iter() {
            match key {
                StylePropertyKey::Padding
                | StylePropertyKey::Margin
                | StylePropertyKey::BorderColor
                | StylePropertyKey::BorderWidth
                | StylePropertyKey::Height
                | StylePropertyKey::Width
                | StylePropertyKey::X
                | StylePropertyKey::Y
                | StylePropertyKey::Display
                | StylePropertyKey::BackgroundColor => {}
                StylePropertyKey::Color
                | StylePropertyKey::FontFamily
                | StylePropertyKey::FontSize => {
                    self.values.insert(key.clone(), value.clone());
                }
            }
        }
    }
}
