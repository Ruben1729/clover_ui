use crate::core::{Node, Style, Bounds};
use crate::core::Context;

pub struct BaseNode {
    style: Style,
    children: Vec<Box<dyn Node>>
}

impl BaseNode {
    pub fn new<F: FnOnce(&mut Vec<Box<dyn Node>>)>(style: Style, add_child: F) -> Box<dyn Node> {
        let mut children = vec![];
        
        add_child(&mut children);
        Box::new(Self {
            style,
            children
        })
    }
}

impl Node for BaseNode {
    fn style(&self) -> &Style {
        &self.style
    }

    fn get_children<'a>(&'a self) -> Box<dyn Iterator<Item = &Box<dyn Node>> + 'a> {
        Box::new(self.children.iter())
    }
    
    fn render(&mut self, ctx: &mut Context) {
        self.style.inherit_position(&ctx.cursor);
        
        if self.style.does_grow() {
            for child in self.children.iter_mut() {
                child.render(ctx);
            }
            
            // here
        } else {
            // here
            
            for child in self.children.iter_mut() {
                child.render(ctx);
            }
        }
        
        self.style.inherit_bounds(&Bounds {
            width: self.style.position.x.clone() - ctx.cursor.x.clone(),
            height: self.style.position.y.clone() - ctx.cursor.y.clone(),
        });
    }
}
