use crate::style::{Color, StyleProperty, StylePropertyKey, StylePropertyMap};

pub type StyleSheet = StylePropertyMap;

impl StyleSheet {
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
