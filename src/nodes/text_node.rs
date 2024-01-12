use crate::core::{BoxModel, Context, Node};
use crate::styles::Style;

pub struct TextNode {
    style: Style,
    value: String
}

impl TextNode {
    pub fn new(value: String, style: Style) -> Box<dyn Node> {
        Box::new(Self {
            style,
            value
        })
    }
}

impl Node for TextNode {
    fn style(&self) -> &Style {
        &self.style
    }
    
    fn get_children<'a>(&'a self) -> Box<dyn Iterator<Item = &Box<dyn Node>> + 'a> {
        Box::new(std::iter::empty())
    }
    fn calculate_size(&mut self) {
        self.style.empty_box_model();
        self.style.box_model.content.width = self.style.content.width.to_pixels();
        self.style.box_model.content.height = self.style.content.height.to_pixels();
    }
    fn render(&mut self, _parent: Option<&BoxModel>, ctx: &mut Context) {
        ctx.draw_text(self.value.clone(), &self.style);
    }
}
