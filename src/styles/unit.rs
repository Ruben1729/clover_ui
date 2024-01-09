use std::ops::{Add, Sub};

#[derive(Default, Debug, Clone)]
pub enum Unit {
    #[default]
    Auto,
    Percent(usize),
    Px(usize)
}

impl Add for Unit {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Unit::Percent(a), Unit::Percent(b)) => Unit::Percent(a + b),
            (Unit::Px(a), Unit::Px(b)) => Unit::Px(a + b),
            (a, b) => panic!("Unable to perform operation {:?} + {:?}", a, b),
        }
    }
}

impl Sub for Unit {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Unit::Percent(a), Unit::Percent(b)) => Unit::Percent(a.checked_sub(b).unwrap_or(0)),
            (Unit::Px(a), Unit::Px(b)) => Unit::Px(a.checked_sub(b).unwrap_or(0)),
            (a, b) => panic!("Unable to perform operation {:?} - {:?}", a, b),
        }
    }
}
