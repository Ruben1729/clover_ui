use std::ops::{Add, Sub};
use crate::styles::Unit;

#[derive(Default, Debug, Clone)]
pub struct Position {
    pub x: Unit,
    pub y: Unit
}

impl Add for Position {
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self {
        Self {
            x: match (self.x, rhs.x) {
                (Unit::Percent(a), Unit::Percent(b)) => Unit::Percent(a + b),
                (Unit::Px(a), Unit::Px(b)) => Unit::Px(a + b),
                (a, b) => panic!("Unable to perform operation {:?} + {:?}", a, b),
            },
            y: match (self.y, rhs.y) {
                (Unit::Percent(a), Unit::Percent(b)) => Unit::Percent(a + b),
                (Unit::Px(a), Unit::Px(b)) => Unit::Px(a + b),
                (a, b) => panic!("Unable to perform operation {:?} + {:?}", a, b),
            },
        }
    }
}

impl Sub for Position {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: match (self.x, rhs.x) {
                (Unit::Percent(a), Unit::Percent(b)) => Unit::Percent(a.checked_sub(b).unwrap_or(0)),
                (Unit::Px(a), Unit::Px(b)) => Unit::Px(a.checked_sub(b).unwrap_or(0)),
                (a, b) => panic!("Unable to perform operation {:?} - {:?}", a, b),
            },
            y: match (self.y, rhs.y) {
                (Unit::Percent(a), Unit::Percent(b)) => Unit::Percent(a.checked_sub(b).unwrap_or(0)),
                (Unit::Px(a), Unit::Px(b)) => Unit::Px(a.checked_sub(b).unwrap_or(0)),
                (a, b) => panic!("Unable to perform operation {:?} - {:?}", a, b),
            },
        }
    }
}
