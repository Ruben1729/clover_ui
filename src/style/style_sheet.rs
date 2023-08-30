use crate::style::{Color, StyleProperty, StylePropertyKey, StylePropertyMap};

pub type StyleSheet = StylePropertyMap;

impl StyleSheet {
    pub fn apply(&mut self, other: &StyleSheet) {
        for (key, value) in other.values.iter() {
            self.values.insert(key.clone(), value.clone());
        }
    }
}
