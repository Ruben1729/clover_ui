mod bounds;
mod color;
mod display;
pub mod preset;
mod position;
mod spacing;
mod unit;

pub use self::bounds::*;
pub use self::color::*;
pub use self::display::*;
pub use self::position::*;
pub use self::spacing::*;
pub use self::unit::*;
use glyphon::{FamilyOwned, Weight};

use crate::core::{BoundsPx, BoxModel};

#[derive(Debug)]
pub struct Style {
    pub position: Position,

    pub display: Display,

    pub background_color: Color,
    pub text_color: Color,

    pub font_weight: Weight,
    pub font_size: Unit,
    pub font_family: FamilyOwned,
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

impl Default for Style {
    fn default() -> Self {
        Style {
            position: Default::default(),
            display: Default::default(),
            background_color: Default::default(),
            text_color: Default::default(),
            font_weight: Weight::NORMAL,
            font_size: Default::default(),
            font_family: FamilyOwned::SansSerif,

            line_height: Unit::Px(10),
            margin: Default::default(),
            padding: Default::default(),
            content: Default::default(),
            x: Default::default(),
            y: Default::default(),
            border: Default::default(),
            border_color: Default::default(),
            box_model: Default::default(),
        }
    }
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
