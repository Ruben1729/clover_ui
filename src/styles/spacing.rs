use crate::core::SpacingPx;
use crate::styles::Unit;

#[derive(Debug, Default, Clone)]
pub struct Spacing {
    pub top: Unit,
    pub right: Unit,
    pub bottom: Unit,
    pub left: Unit,
}

impl Spacing {
    pub fn uniform(width: isize) -> Self {
        Spacing {
            top: Unit::Px(width),
            right: Unit::Px(width),
            bottom: Unit::Px(width),
            left: Unit::Px(width),
        }
    }
    pub fn new(top: Unit, right: Unit, bottom: Unit, left: Unit) -> Self {
        Spacing {
            top,
            right,
            bottom,
            left,
        }
    }
    pub fn to_pixels(&self) -> SpacingPx {
        SpacingPx {
            top: self.top.to_pixels(),
            right: self.right.to_pixels(),
            bottom: self.bottom.to_pixels(),
            left: self.left.to_pixels(),
        }
    }
    pub fn set_horizontal(&mut self, val: Unit) {
        self.right = val.clone();
        self.left = val;
    }

    pub fn set_vertical(&mut self, val: Unit) {
        self.top = val.clone();
        self.bottom = val;
    }

    pub fn set(&mut self, top: Unit, right: Unit, bottom: Unit, left: Unit) {
        self.top = top;
        self.right = right;
        self.bottom = bottom;
        self.left = left;
    }

    pub fn top(&self) -> Unit {
        self.top.clone()
    }

    pub fn bottom(&self) -> Unit {
        self.bottom.clone()
    }

    pub fn right(&self) -> Unit {
        self.right.clone()
    }

    pub fn left(&self) -> Unit {
        self.left.clone()
    }
}