#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub enum Unit {
    #[default]
    Auto,
    Px(isize)
}

impl From<Unit> for isize {
    fn from(val: Unit) -> Self {
        match val {
            Unit::Auto => 0,
            Unit::Px(px) => px
        }
    }
}

impl Unit {
    pub fn to_pixels(&self) -> isize {
        match self {
            Unit::Auto => 0,
            Unit::Px(px) => *px
        }
    }
}
