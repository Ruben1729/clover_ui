use crate::style::{AlignContent, AlignItems, FlexDirection};

#[derive(Debug, Hash, Clone, Copy)]
pub enum Display {
    Block(),
    Flex {
        direction: FlexDirection,
        align_content: AlignContent,
        align_items: AlignItems,
    },
}

impl Default for Display {
    fn default() -> Self {
        Display::Block()
    }
}
