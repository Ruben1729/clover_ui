use crate::style::StyleSheet;
use std::cell::RefCell;
use std::collections::{HashMap};
use std::hash::Hash;
use std::rc::Rc;
use rusttype::Scale;
use uuid::Uuid;
use crate::event::Event;
use crate::paint::{Drawable, Primitive};
use crate::paint::Primitive::Rectangle;

pub enum ElementType {
    Layout,
    Label(String),
}

impl Default for ElementType {
    fn default() -> Self {
        ElementType::Layout
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ElementState {
    Hovered,
    Focused
}

pub struct Element {
    uuid: Uuid,

    pub ty: ElementType,
    pub style: StyleSheet,
    pub state_style: HashMap<ElementState, StyleSheet>,

    parent: Option<Rc<RefCell<Element>>>,
    pub children: Vec<Rc<RefCell<Element>>>,

    pub states: Vec<ElementState>,
}

impl Element {
    pub fn new(
        ty: ElementType,
        parent: Option<Rc<RefCell<Element>>>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            uuid: Uuid::new_v4(),
            ty,
            style: StyleSheet::new(),
            state_style: HashMap::new(),
            parent,
            children: vec![],
            states: Vec::new(),
        }))
    }

    pub fn style(&mut self) -> StyleSheet {
        let base_styles = self.style.values.clone();
        let mut new_style = StyleSheet::new();

        for (_, val) in base_styles.iter() {
            new_style.insert(val.clone());
        }

        for state in &self.states {
            if let Some(style) = self.state_style.get(state) {
                for (_, val) in style.values.iter() {
                    new_style.insert(val.clone());
                }
            }
        }

        new_style
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Element>>) {
        self.children.push(child);
    }

    pub fn is_hovered(&self) -> bool {
        self.states.contains(&ElementState::Hovered)
    }

    pub fn handle_event(&mut self, event: &Event) {
        let curr_style = self.style();
        match event {
            Event::KeyDown(_) => {}
            Event::KeyUp(_) => {}
            Event::MouseScroll(_) => {}
            Event::MouseEnter => {}
            Event::MouseLeave => {}
            Event::MouseMove(move_event) => {
                if let Some(position) = move_event {
                    if position.0 > curr_style.get_x() as f32
                        && position.0 < (curr_style.get_x() + curr_style.get_total_width()) as f32
                        && position.1 > curr_style.get_y() as f32
                        && position.1 < (curr_style.get_y() + curr_style.get_total_height()) as f32
                    {
                        if !self.states.contains(&ElementState::Hovered) {
                            self.states.push(ElementState::Hovered);
                        }
                    } else {
                        self.states.retain(|state| state != &ElementState::Hovered);
                    }
                } else {
                    self.states.retain(|state| state != &ElementState::Hovered);
                }
            }
            Event::Click(_) => {}
            Event::MouseDown(_) => {}
            Event::MouseUp(_) => {}
        }
    }
}

impl Drawable for Element {
    fn draw(&mut self) -> Vec<Primitive> {
        let mut primitives = Vec::new();
        let style = self.style();

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

        match &self.ty {
            ElementType::Layout  => {}
            ElementType::Label(value) => {
                primitives.push(Primitive::Text {
                    x: style.get_content_x() as f32,
                    y: style.get_content_y() as f32,
                    scale: Scale {
                        x: style.get_fontsize(),
                        y: style.get_fontsize(),
                    },
                    content: value.clone(),
                    color: style.get_color().get_u32(),
                });
            }
        }

        primitives
    }
}
