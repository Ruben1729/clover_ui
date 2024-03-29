use crate::core::{BoxModel, Node, PositionPx};
use crate::styles::{Style, Display, Position, Unit};
use crate::core::Context;

pub struct BaseNode {
    style: Style,
    children: Vec<Box<dyn Node>>,

    hovered: bool,

    pub mouse_enter_handler: Box<dyn Fn(&mut Style, &PositionPx)>,
    pub mouse_leave_handler: Box<dyn Fn(&mut Style, &PositionPx)>,
    pub mouse_over_handler:  Box<dyn Fn(&mut Style, &PositionPx)>,
}

impl BaseNode {
    pub fn new<F: FnOnce(&mut Vec<Box<dyn Node>>)>
    (style: Style, add_child: F) -> BaseNode {
        let mut children = vec![];
        
        add_child(&mut children);
        Self {
            style,
            children,
            hovered: false,
            mouse_enter_handler: Box::new(|_style, _position| {}),
            mouse_leave_handler: Box::new(|_style, _position| {}),
            mouse_over_handler: Box::new(|_style, _position| {})
        }
    }

    pub fn build(self) -> Box<dyn Node> {
        Box::new(self)
    }
}

impl Node for BaseNode {
    fn style(&self) -> &Style {
        &self.style
    }

    fn get_children<'a>(&'a self) -> Box<dyn Iterator<Item = &Box<dyn Node>> + 'a> {
        Box::new(self.children.iter())
    }

    fn render(&mut self, parent_opt: Option<&BoxModel>, ctx: &mut Context) {
        match self.style.display {
            Display::Block => {
                let underflow = match parent_opt {
                    None => 0,
                    Some(parent) => parent.content.width - self.style.box_model.width()
                };

                match self.style.position {
                    Position::Static => {
                        self.style.box_model.y = ctx.cursor.y;
                        self.style.box_model.x = ctx.cursor.x;

                        match (underflow > 0, self.style.margin.left == Unit::Auto, self.style.margin.right == Unit::Auto) {
                            (true, true, true) => {
                                let new_margin = underflow / 2;
                                self.style.box_model.margin.set_horizontal(new_margin);
                            },
                            (true, true, false) => {
                                self.style.box_model.margin.left = underflow;
                            },
                            (true, false, true) => {
                                self.style.box_model.margin.right = underflow;
                            },
                            _ => {}
                        }

                        ctx.cursor.y +=
                            self.style.box_model.margin.top +
                            self.style.box_model.border.top +
                            self.style.box_model.padding.top;
                    }
                    Position::Relative(_, _) => {}
                    Position::Fixed(_, _) => {}
                    Position::Absolute(_, _) => {}
                    Position::Sticky(_, _) => {}
                }

                for child in &mut self.children {
                    child.render(Some(&self.style.box_model), ctx);
                }

                match self.style.position {
                    Position::Static => {
                        ctx.cursor.y +=
                            self.style.box_model.content.height +
                            self.style.box_model.margin.bottom +
                            self.style.box_model.border.bottom +
                            self.style.box_model.padding.bottom;
                    }
                    Position::Relative(_, _) => {}
                    Position::Fixed(_, _) => {}
                    Position::Absolute(_, _) => {}
                    Position::Sticky(_, _) => {}
                }

                ctx.draw_box(&self.style);
            }
            Display::None => {}
            Display::Flex(_) => {}
            Display::Grid => {}
        }
    }
    fn calculate_size(&mut self) {
        self.style.empty_box_model();

        for child in &mut self.children {
            child.calculate_size();

            if child.style().position == Position::Static {
                if self.style.box_model.content.width < child.style().box_model.width() {
                    self.style.box_model.content.width = child.style().box_model.width();
                }

                self.style.box_model.content.height += child.style().box_model.height();
            }
        }

        self.style.box_model.content.height = match self.style.content.height {
            Unit::Auto => self.style.box_model.content.height,
            Unit::Px(px) => px
        };

        self.style.box_model.content.width = match self.style.content.width {
            Unit::Auto => self.style.box_model.content.width,
            Unit::Px(px) => px
        };
    }
    fn on_cursor_move(&mut self, position: &PositionPx) {
        match (self.hovered,
                self.style.box_model.x < position.x &&
                self.style.box_model.width() > position.x &&
                self.style.box_model.y < position.y &&
                self.style.box_model.height() > position.y
        ) {
            (false, true) => {
                self.hovered = true;

                for child in &mut self.children {
                    child.on_cursor_move(position);
                }

                (self.mouse_enter_handler)(&mut self.style, position);
            },
            (true, false) => {
                self.hovered = false;

                for child in &mut self.children {
                    child.on_cursor_move(position);
                }

                (self.mouse_leave_handler)(&mut self.style, position);
            },
            (true, true) => {
                for child in &mut self.children {
                    child.on_cursor_move(position);
                }

               (self.mouse_over_handler)(&mut self.style, position);
            }
            _ => {}
        }
    }
}
