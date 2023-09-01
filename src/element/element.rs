use crate::element::{ElementState, ElementStateManager};
use crate::event::Event;
use crate::paint::Primitive::Rectangle;
use crate::paint::{Drawable, Primitive};
use crate::state::MouseButton;
use crate::style::{Layout, StyleSheet};
use std::collections::HashMap;
use std::hash::Hash;
use uuid::Uuid;

#[derive(Debug)]
pub enum ElementType<'a> {
    Container(Layout),
    Label(&'a str),
    TextInput(&'a mut String),
}

impl<'a> Default for ElementType<'a> {
    fn default() -> Self {
        ElementType::Container(Layout::default())
    }
}

pub struct Element<'a> {
    uuid: Uuid,
    pub ty: ElementType<'a>,
    pub style: StyleSheet,
    pub state_manager: ElementStateManager,
    pub state_style: HashMap<ElementState, StyleSheet>,
}

impl<'a> Element<'a> {
    pub fn new(ty: ElementType<'a>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            ty,
            style: StyleSheet::new(),
            state_manager: Default::default(),
            state_style: Default::default(),
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
        match event {
            Event::KeyDown(_) => {}
            Event::KeyUp(_) => {}
            Event::MouseScroll(_) => {}
            Event::MouseMove(state) => {
                if let Some((x, y)) = state.pos {
                    if self.cursor_in_bounds(x, y) {
                        self.state_manager.push(ElementState::Hovered);
                    } else {
                        self.state_manager.remove(ElementState::Hovered);
                    }
                } else {
                    self.state_manager.remove(ElementState::Hovered);
                }
            }
            Event::MouseDown { state, .. } => {
                if let Some(position) = state.pos {
                    if self.cursor_in_bounds(position.0, position.1) {
                        self.state_manager.push(ElementState::MouseDown);
                    }
                } else {
                    self.state_manager.remove(ElementState::MouseDown);
                }
            }
            Event::MouseUp { state, button } => {
                if let Some((x, y)) = state.pos {
                    if self.cursor_in_bounds(x, y) {
                        match button {
                            MouseButton::Left => {
                                self.state_manager.push(ElementState::Clicked);
                            }
                            MouseButton::Middle => {}
                            MouseButton::Right => {}
                        }
                    }
                }
                self.state_manager.remove(ElementState::MouseDown);
            }
        }
    }

    pub fn cursor_in_bounds(&mut self, x: f32, y: f32) -> bool {
        let curr_style = self.style();
        x > curr_style.get_x() as f32
            && x < (curr_style.get_x() + curr_style.get_total_width()) as f32
            && y > curr_style.get_y() as f32
            && y < (curr_style.get_y() + curr_style.get_total_height()) as f32
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
            ElementType::Container(_) => {}
            ElementType::Label(value) => {
                primitives.push(Primitive::Text {
                    x: style.get_content_x() as f32,
                    y: style.get_content_y() as f32,
                    font_weight: style.get_fontweight(),
                    font_size: style.get_fontsize(),
                    content: value.to_string(),
                    color: style.get_color().get_u32(),
                });
            }
            ElementType::TextInput(_) => {}
        }

        primitives
    }
}
