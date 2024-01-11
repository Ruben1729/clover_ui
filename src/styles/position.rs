use super::Unit;

#[derive(Default, Debug, Eq, PartialEq)]
pub enum Position {
    #[default]
    Static,
    Relative(Unit, Unit),
    Fixed(Unit, Unit),
    Absolute(Unit, Unit),
    Sticky(Unit, Unit)
}