mod flex;

pub use self::flex::*;

#[derive(Debug, Clone)]
pub enum Layout {
    Block,
    Flex {
        flex_direction: FlexDirection
    }
}

impl Default for Layout {
    fn default() -> Self {
        Layout::Block
    }
}
