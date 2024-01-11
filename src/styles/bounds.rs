use crate::core::BoundsPx;
use crate::styles::Unit;

#[derive(Default, Debug, Clone)]
pub struct Bounds {
    pub width: Unit,
    pub height: Unit
}

impl Bounds {
    pub fn new(width: Unit, height: Unit) -> Self {
        Self {
            width,
            height
        }
    }
    pub fn to_pixels(&self) -> BoundsPx {
        BoundsPx {
            width: self.width.to_pixels(),
            height: self.height.to_pixels()
        }
    }
    pub fn px(width: isize, height: isize) -> Self {
        Self {
            width: Unit::Px(width),
            height: Unit::Px(height)
        }
    }
    pub fn width(&self) -> Unit {
        self.width.clone()
    }

    pub fn height(&self) -> Unit {
        self.height.clone()
    }
}
