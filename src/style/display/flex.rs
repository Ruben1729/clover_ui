#[derive(Debug, Hash, Clone, Copy)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Col,
    ColReverse,
}

#[derive(Debug, Hash, Clone, Copy)]
pub enum AlignContent {
    Start,
    Center,
    SpaceBetween,
    SpaceAround,
}

#[derive(Debug, Hash, Clone, Copy)]
pub enum AlignItems {
    Stretch,
    Center,
    Start,
    End,
}
