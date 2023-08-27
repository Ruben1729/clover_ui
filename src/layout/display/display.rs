use crate::layout::FlexProperties;

#[derive(Debug)]
pub enum Display {
    Block(),
    Flex(FlexProperties),
}

impl Default for Display {
    fn default() -> Self {
        Display::Block()
    }
}
