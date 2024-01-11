use crate::styles::Unit;

#[derive(Debug, Default)]
pub struct FlexConfig {
    direction: FlexDirection,
    gap: Unit
}

#[derive(Default, Debug)]
pub enum FlexDirection {
    #[default]
    Row,
    RowReverse,
    Column,
    ColumnReverse
}