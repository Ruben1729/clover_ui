#[derive(Debug)]
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