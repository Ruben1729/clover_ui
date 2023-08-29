use crate::style::{Border, Color, Spacing, StyleProperty, StyleSheet};

#[derive(Default, Clone)]
pub struct Style {
    pub padding: Spacing,
    pub margin: Spacing,
    pub border: Border,
    pub height: usize,
    pub width: usize,
    pub y: usize,
    pub x: usize,

    // pub display: Display,

    pub background_color: Color,
    pub color: Color,

    pub font_family: Option<String>,
    pub font_size: f32,
}

impl Style {
    pub fn apply(&mut self, styles: &StyleSheet) {
        for curr_style in &styles.0 {
            match curr_style {
                StyleProperty::Padding(val) => {
                    self.padding = val.clone();
                }
                StyleProperty::Margin(val) => {
                    self.margin = val.clone();
                }
                StyleProperty::Border(val) => {
                    self.border = val.clone();
                }
                StyleProperty::Height(val) => {
                    self.height = val.clone();
                }
                StyleProperty::Width(val) => {
                    self.width = val.clone();
                }
                StyleProperty::X(val) => {
                    self.x = val.clone();
                }
                StyleProperty::Y(val) => {
                    self.y = val.clone();
                }
                StyleProperty::BackgroundColor(val) => {
                    self.background_color = val.clone();
                }
                StyleProperty::Color(val) => {
                    self.color = val.clone();
                }
                StyleProperty::FontFamily(val) => self.font_family = val.clone(),
                StyleProperty::FontSize(val) => self.font_size = val.clone(),
            }
        }
    }
}
