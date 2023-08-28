#[derive(Debug, Hash, Clone, Copy)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Col,
    ColReverse
}

#[derive(Debug)]
pub struct FlexProperties {
    pub direction: FlexDirection
}