mod primitives;
mod painter;

pub use self::primitives::*;
pub use self::painter::*;

pub trait Drawable {
    fn draw(&self) -> Vec<Primitive>;
}
