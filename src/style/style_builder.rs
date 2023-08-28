use crate::style::{Border, Color, Display, Style, Spacing};

#[derive(Debug, Default)]
pub struct StyleBuilder {
    padding:            Spacing,
    margin:             Spacing,
    border:             Border,
    height:             usize,
    width:              usize,
    display:            Display,
    background_color:   Color,
    color:              Color
}

impl StyleBuilder {
    pub fn with_padding(mut self, top: usize, right: usize, bottom: usize, left: usize) -> Self {
        self.padding.set(top, right, bottom, left);
        self
    }

    pub fn with_margin(mut self, top: usize, right: usize, bottom: usize, left: usize) -> Self {
        self.margin.set(top, right, bottom, left);
        self
    }
    pub fn with_border_width(mut self, width: usize) -> Self {
        self.border.set_width(width);
        self
    }
    pub fn with_border_color(mut self, color: Color) -> Self {
        self.border.set_color(color);
        self
    }
    pub fn with_height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }
    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }
    pub fn with_display(mut self, display: Display) -> Self {
        self.display = display;
        self
    }
    pub fn with_background_color(mut self, color: Color) -> Self {
        self.background_color.set_u32(color.get_u32());
        self
    }
    pub fn with_color(mut self, color: Color) -> Self {
        self.color.set_u32(color.get_u32());
        self
    }
    pub fn build(self) -> Style {
        Style {
            padding: self.padding,
            margin: self.margin,
            border: self.border,
            height: self.height,
            width: self.width,
            x: 0,
            y: 0,
            display: self.display,
            background_color: self.background_color,
            color: self.color,
        }
    }
}
