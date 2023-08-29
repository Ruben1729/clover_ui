use crate::style::{Border, Color, Display, Spacing};
use rusttype::Font;

#[derive(Debug, Clone)]
pub enum StyleProperty {
    Padding(Spacing),
    Margin(Spacing),
    Border(Border),
    Height(usize),
    Width(usize),
    X(usize),
    Y(usize),
    Display(Display),
    BackgroundColor(Color),
    Color(Color),
    Font(Option<String>),
    FontSize(f32),
}

pub type ConditionalStyle = Vec<StyleProperty>;

#[derive(Default, Clone)]
pub struct Style {
    pub padding: Spacing,
    pub margin: Spacing,
    pub border: Border,
    pub height: usize,
    pub width: usize,
    pub y: usize,
    pub x: usize,

    pub display: Display,

    pub background_color: Color,
    pub color: Color,

    pub font: Option<String>,
    pub font_size: f32,
}

impl Style {
    pub fn content_x(&self) -> usize {
        self.x + self.margin.left + self.border.left + self.padding.left
    }

    pub fn content_y(&self) -> usize {
        self.y + self.margin.top + self.border.top + self.padding.top
    }

    pub fn width(&self) -> usize {
        self.margin.horizontal() + self.border.horizontal() + self.padding.horizontal() + self.width
    }

    pub fn height(&self) -> usize {
        self.margin.vertical() + self.border.vertical() + self.padding.vertical() + self.height
    }

    pub fn color_at_px(&self, dx: usize, dy: usize) -> Option<u32> {
        return if dx > self.margin.left + self.border.left
            && dx
                < self.margin.left
                    + self.border.left
                    + self.padding.left
                    + self.width
                    + self.padding.right
            && dy > self.margin.top + self.border.top
            && dy
                < self.margin.top
                    + self.border.top
                    + self.padding.top
                    + self.height
                    + self.padding.bottom
        {
            Some(self.background_color.get_u32())
        } else if dx > self.margin.left
            && dx < self.width() - self.margin.right
            && dy > self.margin.top
            && dy < self.height() - self.margin.bottom
        {
            Some(self.border.color().0)
        } else {
            None
        };
    }
}
