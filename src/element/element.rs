use std::collections::HashMap;
use crate::paint::Primitive::Rectangle;
use crate::paint::{Drawable, Primitive};
use crate::style::StyleSheet;
use rusttype::Scale;
use std::hash::Hash;
use uuid::Uuid;
use crate::element::{ElementState, ElementStateManager};
use crate::event::Event;

#[derive(Debug)]
pub enum ElementType<'a> {
    Layout,
    Label(&'a str),
    TextInput(&'a mut String),
}

impl<'a> ElementType<'a> {
    pub fn to_key(&self) -> &str {
        match self {
            ElementType::Layout =>          "lay",
            ElementType::Label(_) =>        "lab",
            ElementType::TextInput(_) =>    "inp"
        }
    }
}

impl<'a> Default for ElementType<'a> {
    fn default() -> Self {
        ElementType::Layout
    }
}

pub struct Element<'a> {
    uuid: Uuid,
    pub ty: ElementType<'a>,
    pub style: StyleSheet,
    pub state_manager: ElementStateManager,
    pub state_style: HashMap<ElementState, StyleSheet>
}

impl<'a> Element<'a> {
    pub fn new(ty: ElementType<'a>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            ty,
            style: StyleSheet::new(),
            state_manager: Default::default(),
            state_style: Default::default()
        }
    }

    pub fn style(&mut self) -> StyleSheet {
        let base_styles = self.style.values.clone();
        let mut new_style = StyleSheet::new();

        for (_, val) in base_styles.iter() {
            new_style.insert(val.clone());
        }

        for state in &self.state_manager.states {
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

    pub fn handle_event(&mut self, event: &Event) {
        let curr_style = self.style();
        match event {
            Event::KeyDown(_) => {}
            Event::KeyUp(_) => {}
            Event::MouseScroll(_) => {}
            Event::MouseMove(move_event) => {
                if let Some(position) = move_event {
                    if position.0 > curr_style.get_x() as f32
                        && position.0 < (curr_style.get_x() + curr_style.get_total_width()) as f32
                        && position.1 > curr_style.get_y() as f32
                        && position.1 < (curr_style.get_y() + curr_style.get_total_height()) as f32 {
                        self.state_manager.push(ElementState::Hovered);
                    }
                } else {
                    self.state_manager.remove(ElementState::Hovered);
                }
            }
            Event::Click(btn) => {

            }
            Event::MouseDown(_) => {}
            Event::MouseUp(_) => {}
        }
    }
}

impl<'a> Drawable for Element<'a> {
    fn draw(&mut self) -> Vec<Primitive> {
        let mut primitives = Vec::new();
        let style = self.style();

        primitives.push(Rectangle {
            x: (style.get_x() + style.get_margin().left) as f32,
            y: (style.get_y() + style.get_margin().top) as f32,
            width: (style.get_borderwidth().horizontal()
                + style.get_padding().horizontal()
                + style.get_width()) as f32,
            height: (style.get_borderwidth().vertical()
                + style.get_padding().vertical()
                + style.get_height()) as f32,
            color: style.get_bordercolor().get_u32(),
        });
        primitives.push(Rectangle {
            x: (style.get_x() + style.get_margin().left + style.get_borderwidth().left) as f32,
            y: (style.get_y() + style.get_margin().top + style.get_borderwidth().top) as f32,
            width: (style.get_padding().horizontal() + style.get_width()) as f32,
            height: (style.get_padding().vertical() + style.get_height()) as f32,
            color: style.get_backgroundcolor().get_u32(),
        });

        match self.ty {
            ElementType::Layout => {}
            ElementType::Label(value) => {
                primitives.push(Primitive::Text {
                    x: style.get_content_x() as f32,
                    y: style.get_content_y() as f32,
                    scale: Scale {
                        x: style.get_fontsize(),
                        y: style.get_fontsize(),
                    },
                    content: value.to_string(),
                    color: style.get_color().get_u32(),
                });
            }
            ElementType::TextInput(_) => {}
        }

        primitives
    }
}
