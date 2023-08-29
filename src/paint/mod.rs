mod painter;
mod primitives;

pub use self::painter::*;
pub use self::primitives::*;

pub trait Drawable {
    fn draw(&self) -> Vec<Primitive>;
}
