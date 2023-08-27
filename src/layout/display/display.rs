use crate::layout::{FlexDirection, FlexProperties};

#[derive(Debug)]
pub enum Display {
    Block(),
    Flex { direction: FlexDirection },
}

impl Default for Display {
    fn default() -> Self {
        Display::Block()
    }
}
