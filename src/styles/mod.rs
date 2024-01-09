mod bounds;
mod color;
mod position;
mod unit;

pub use self::bounds::*;
pub use self::color::*;
pub use self::position::*;
pub use self::unit::*;

#[derive(Default, Debug)]
pub struct Style {
    pub position: Position,
    pub bounds: Bounds,

    pub background_color: Color,
    pub text_color: Color
}

impl Style {
    pub fn does_grow(&self) -> bool {
        let grows_x = match self.bounds.width {
            Unit::Auto => true,
            _ => false
        };
        
        let grows_y = match self.bounds.height {
            Unit::Auto => true,
            _ => false
        };
        
        grows_x || grows_y
    }
    
    pub fn is_fixed(&self) -> bool {
        let x = match self.position.x {
            Unit::Auto => false,
            _ => true
        };
        
        let y = match self.position.y {
            Unit::Auto => false,
            _ => true
        };
        
        x || y
    }
    
    pub fn inherit_position(&mut self, new_position: &Position) {
        self.position.x = match self.position.x {
            Unit::Auto => new_position.x.clone(),
            _ => self.position.x.clone()
        };

        self.position.y = match self.position.y {
            Unit::Auto => new_position.y.clone(),
            _ => self.position.y.clone()
        };
    }
    
    pub fn inherit_bounds(&mut self, new_bounds: &Bounds) {
        self.bounds.width = match self.bounds.width {
            Unit::Auto => new_bounds.width.clone(),
            _ => self.bounds.width.clone()
        };
        
        self.bounds.height = match self.bounds.height {
            Unit::Auto => new_bounds.height.clone(),
            _ => self.bounds.height.clone()
        };
    }
}
