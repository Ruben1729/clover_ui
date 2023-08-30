mod align;
mod flex;

pub use self::align::*;
pub use self::flex::*;

#[derive(Debug, Clone)]
pub enum Layout {
    Block,
    InlineBlock,
    Hidden,
    Flex {
        flex_direction: FlexDirection,
        align_content: AlignContent,
        align_items: AlignItems,
    },
    Grid {
        align_content: AlignContent,
        align_items: AlignItems,
    },
}

impl Default for Layout {
    fn default() -> Self {
        Layout::Block
    }
}
