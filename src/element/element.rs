use crate::style::StyleSheet;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use uuid::Uuid;
use crate::event::Event;
use crate::paint::{Drawable, Primitive};
use crate::paint::Primitive::Rectangle;

pub enum ElementType {
    Layout,
    Label,
    Button,
}

impl Default for ElementType {
    fn default() -> Self {
        ElementType::Layout
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ElementState {
    Hovered,
}

pub struct Element {
    pub id: String,
    uuid: Uuid,
    pub ty: ElementType,
    pub style: StyleSheet,

    parent: Option<Rc<RefCell<Element>>>,
    pub children: Vec<Rc<RefCell<Element>>>,

    pub state: HashSet<ElementState>,
}

impl Element {
    pub fn new(
        id: String,
        ty: ElementType,
        parent: Option<Rc<RefCell<Element>>>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            id,
            uuid: Uuid::new_v4(),
            ty,
            style: StyleSheet::new(),
            parent,
            children: vec![],
            state: HashSet::new(),
        }))
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Element>>) {
        self.children.push(child);
    }

    pub fn is_hovered(&self) -> bool {
        self.state.contains(&ElementState::Hovered)
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown(_) => {}
            Event::KeyUp(_) => {}
            Event::MouseScroll(_) => {}
            Event::MouseEnter => {}
            Event::MouseLeave => {}
            Event::MouseMove(move_event) => {
                if let Some(position) = move_event {
                    if position.0 > self.style.get_x() as f32
                        && position.0 < (self.style.get_x() + self.style.get_total_width()) as f32
                        && position.1 > self.style.get_y() as f32
                        && position.1 < (self.style.get_y() + self.style.get_total_height()) as f32
                    {
                        self.state.insert(ElementState::Hovered);
                    } else {
                        self.state.remove(&ElementState::Hovered);
                    }
                } else {
                    self.state.remove(&ElementState::Hovered);
                }
            }
            Event::Click(_) => {}
            Event::MouseDown(_) => {}
            Event::MouseUp(_) => {}
        }
    }
}

impl Drawable for Element {
    fn draw(&self) -> Vec<Primitive> {
        let mut primitives = Vec::new();
        let style = &self.style;

        primitives.push(Rectangle {
            x: (style.get_x() + style.get_margin().left) as f32,
            y: (style.get_y() + style.get_margin().top) as f32,
            width: (style.get_borderwidth().horizontal() + style.get_padding().horizontal() + style.get_width()) as f32,
            height: (style.get_borderwidth().vertical() + style.get_padding().vertical() + style.get_height()) as f32,
            color: style.get_bordercolor().get_u32(),
        });
        primitives.push(Rectangle {
            x: (style.get_x() + style.get_margin().left + style.get_borderwidth().left) as f32,
            y: (style.get_y() + style.get_margin().top + style.get_borderwidth().top) as f32,
            width: (style.get_padding().horizontal() + style.get_width()) as f32,
            height: (style.get_padding().vertical() + style.get_height()) as f32,
            color: style.get_backgroundcolor().get_u32(),
        });

        primitives
    }
}
