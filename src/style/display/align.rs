
#[derive(Debug, Clone)]
pub enum AlignContent {
    Start,
    Center,
    SpaceBetween,
    SpaceAround
}

impl Default for AlignContent {
    fn default() -> Self {
        AlignContent::Start
    }
}

#[derive(Debug, Clone)]
pub enum AlignItems {
    Stretch,
    Center,
    Start,
    End
}

impl Default for AlignItems {
    fn default() -> Self {
        AlignItems::Stretch
    }
}