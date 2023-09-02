#[derive(Debug, Clone)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Col,
    ColReverse,
}

impl Default for FlexDirection {
    fn default() -> Self {
        FlexDirection::Row
    }
}

#[derive(Debug, Default, Clone)]
pub struct Layout {
    pub(crate) flex_direction: FlexDirection,
}
