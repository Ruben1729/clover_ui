use std::collections::{HashMap, VecDeque};
use crate::paint::{Drawable, Primitive};
use crate::style::{FlexDirection, Layout, StyleSheet};
use uuid::Uuid;
use crate::context::Context;
use crate::element::{Element, ElementStateManager, ElementType};

pub struct Ui {
    pub style_stack:                Vec<(Option<Uuid>, StyleSheet)>,
    pub draw_calls:                 Vec<Primitive>,
    pub parent_layout:              VecDeque<Layout>,
    pub running_counter:            usize,
    pub persistent_element_state:   HashMap<usize, ElementStateManager>,

    pub context:                    Context,
    pub cursor:                     (usize, usize),
}

impl Default for Ui {
    fn default() -> Self {
        Ui {
            style_stack: Vec::new(),
            draw_calls: Vec::new(),
            parent_layout: vec![Layout::default()].into(),
            running_counter: 0,
            persistent_element_state: HashMap::new(),

            context: Context::default(),
            cursor: (0, 0),
        }
    }
}

impl Ui {
    pub fn clear(&mut self) {
        self.context.event_queue.truncate(0);
        self.cursor = (0,0);
        self.draw_calls.truncate(0);
        self.running_counter = 0;
    }
    pub fn move_to_cursor(&mut self, new_element: &mut Element) {
        if let Some(layout) = self.parent_layout.front() {
            let (new_x, new_y) = match layout.flex_direction {
                FlexDirection::Row
                | FlexDirection::Col => {(
                    self.cursor.0,
                    self.cursor.1
                )}
                FlexDirection::RowReverse => {(
                    self.cursor.0 - new_element.style.get_width(),
                    self.cursor.1
                )}
                FlexDirection::ColReverse => {(
                    self.cursor.0,
                    self.cursor.1 - new_element.style.get_height()
                )}
            };

            self.cursor = (new_x, new_y);
            new_element.style.set_x(new_x);
            new_element.style.set_y(new_y);
        } else {
            panic!("Tried updating cursor without specifying the layout.");
        }
    }

    pub fn move_cursor_to_next_element(&mut self, new_element: &Element) {
        if let Some(layout) = self.parent_layout.front() {
            self.cursor = match layout.flex_direction {
                FlexDirection::Row => {(
                    self.cursor.0 + new_element.style.get_width(),
                    self.cursor.1
                    )}
                FlexDirection::Col => {(
                    self.cursor.0,
                    self.cursor.1 + new_element.style.get_height()
                )}
                FlexDirection::RowReverse
                | FlexDirection::ColReverse => {(
                    self.cursor.0,
                    self.cursor.1
                )}
            };
        } else {
            panic!("Tried updating cursor without specifying the layout.");
        }
    }
    pub fn move_cursor_to_content(&mut self, new_element: &Element) {
        if let Some(layout) = self.parent_layout.front() {
            self.cursor = match layout.flex_direction {
                FlexDirection::Row | FlexDirection::Col => {
                    (
                        new_element.style.get_content_x(),
                        new_element.style.get_content_y()
                    )
                }
                FlexDirection::RowReverse => {
                    (
                        new_element.style.get_content_x() + new_element.style.get_width(),
                        new_element.style.get_content_y()
                    )
                }
                FlexDirection::ColReverse => {
                    (
                        new_element.style.get_content_x(),
                        new_element.style.get_content_y() + new_element.style.get_height()
                    )
                }
            };

        } else {
            panic!("Tried updating cursor without specifying the layout.");
        }
    }

    // Function inherits style unless the top of the stack is unclaimed
    // In that case, the element claims ownership of that style and applies it
    pub fn inherit_style(&mut self, new_element: &mut Element) {
        for (owner_opt, sheet) in self.style_stack.iter_mut() {
            if let Some(_) = owner_opt {
                new_element.style.inherit(sheet);
            } else {
                *owner_opt = Some(new_element.uuid());
                new_element.style.apply(sheet);
            }
        }
    }
    pub fn unbind_styles(&mut self, owner_uuid: Uuid) {
        if !self.style_stack.is_empty() {
            if let Some(uuid) = self.style_stack.last().unwrap().0 {
                if owner_uuid == uuid {
                    self.style_stack.pop();
                }
            }
        }
    }
    pub fn with_style_sheet(&mut self, styles: StyleSheet) -> &mut Self {
        self.style_stack.push((None, styles));
        self
    }
}
