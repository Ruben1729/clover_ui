mod bounds;
mod color;
mod display;
pub mod preset;
mod font_db;
mod font_family;
mod font_weight;
mod position;
mod spacing;
mod unit;

pub use self::bounds::*;
pub use self::color::*;
pub use self::display::*;
pub use self::font_family::*;
pub use self::font_weight::*;
pub use self::position::*;
pub use self::spacing::*;
pub use self::unit::*;

use crate::core::{BoundsPx, BoxModel};

#[derive(Default, Debug)]
pub struct Style {
    pub position: Position,

    pub display: Display,

    pub background_color: Color,
    pub text_color: Color,

    pub font_weight: FontWeight,
    pub font_size: Unit,
    pub font_family: FontFamily,

    pub line_height: Unit,

    pub margin: Spacing,
    pub padding: Spacing,
    pub content: Bounds,
    pub x: Unit,
    pub y: Unit,

    pub border: Spacing,
    pub border_color: Color,

    pub box_model: BoxModel
}

impl Style {
    pub fn empty_box_model(&mut self) {
        self.box_model = BoxModel {
            margin: self.margin.to_pixels(),
            border: self.border.to_pixels(),
            padding: self.padding.to_pixels(),
            content: BoundsPx {
                width: 0,
                height: 0
            },
            x: self.x.to_pixels(),
            y: self.y.to_pixels(),
        };
    }
}
