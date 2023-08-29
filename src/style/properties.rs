use crate::style::{Border, Color, Spacing};

#[derive(Debug, Clone)]
pub enum StyleProperty {
    Padding(Spacing),
    Margin(Spacing),
    Border(Border),
    Height(usize),
    Width(usize),
    X(usize),
    Y(usize),
    // Display(Display),
    BackgroundColor(Color),
    Color(Color),
    FontFamily(Option<String>),
    FontSize(f32),
}
