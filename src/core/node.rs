pub use crate::styles::*;
use crate::core::context::Context;

pub trait Node {
    fn style(&self) -> &Style;
    fn get_children<'a>(&'a self) -> Box<dyn Iterator<Item = &Box<dyn Node>> + 'a>;
    fn render(&mut self, ctx: &mut Context) {}
}
