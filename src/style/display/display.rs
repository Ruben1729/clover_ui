use crate::style::{FlexDirection, FlexProperties};

#[derive(Debug, Hash, Clone, Copy)]
pub enum Display {
    Block(),
    Flex { direction: FlexDirection },
}

impl Default for Display {
    fn default() -> Self {
        Display::Block()
    }
}
