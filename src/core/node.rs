use crate::core::BoxModel;
use crate::styles::Style;
use crate::core::context::Context;

pub trait Node {
    fn style(&self) -> &Style;
    fn get_children<'a>(&'a self) -> Box<dyn Iterator<Item = &Box<dyn Node>> + 'a>;
    fn render(&mut self, _parent: Option<&BoxModel>, _ctx: &mut Context) {}
    fn calculate_size(&mut self) {}
}
