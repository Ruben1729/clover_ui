pub mod flex;

use flex::*;

#[derive(Default, Debug)]
pub enum Display {
    #[default]
    Block,
    None,
    Flex(FlexConfig),
    Grid
}
