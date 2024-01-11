#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub enum Unit {
    #[default]
    Auto,
    Px(isize)
}

impl Into<isize> for Unit {
    fn into(self) -> isize {
        match self {
            Unit::Auto => 0,
            Unit::Px(px) => px
        }
    }
}

impl Unit {
    pub fn to_pixels(&self) -> isize {
        match self {
            Unit::Auto => 0,
            Unit::Px(px) => px.clone()
        }
    }
}
