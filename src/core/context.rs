use crate::styles::{Bounds, Position, Style};
use crate::core::render::Primitive;

#[derive(Default, Debug)]
pub struct Context {
    pub available_bounds: Bounds,
    pub cursor: Position,
    
    pub render_calls: Vec<Primitive>
}

impl Context {
    pub fn draw(style: &Style) {
        
    }
}
