use crate::element::{Element, ElementType};
use crate::style::StyleSheet;
use std::cell::{RefCell};
use std::collections::VecDeque;
use std::rc::Rc;
use uuid::Uuid;
use crate::paint::{Drawable, Primitive};

pub struct Ui<'a> {
    pub root_idx: usize,
    pub current_parent: VecDeque<usize>,
    pub style_stack: Vec<(Option<Uuid>, StyleSheet)>,
    pub dom_tree: Vec<Element<'a>>
}

impl<'a> Default for Ui<'a> {
    fn default() -> Self {
        let root = Element::new(ElementType::Layout, None);
        let mut dom_tree = Vec::new();
        dom_tree.push(root);

        let root_idx = dom_tree.len() - 1;
        Ui {
            root_idx,
            current_parent: vec![root_idx].into(),
            style_stack: Vec::new(),
            dom_tree
        }
    }
}

impl<'a> Ui<'a> {
    pub fn add_element(&mut self, new_element: Element<'a>) -> usize {
        if let Some(parent_idx) = self.current_parent.front() {
            if *parent_idx >= self.dom_tree.len() {
                panic!("Invalid parent index.")
            }

            self.dom_tree.push(new_element);
            let element_idx = self.dom_tree.len() - 1;
            self.dom_tree[*parent_idx].children.push(element_idx);
            element_idx
        } else {
            panic!("Tried inserting an element into a non existing parent.")
        }
    }
    pub fn style_element(&mut self, new_element: &mut Element) {
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

    pub fn traverse_and_print(&self) {
        self._traverse_and_print(self.root_idx, 0);
    }

    fn _traverse_and_print(&self, element_idx: usize, level: usize) {
        let indent = " ".repeat(level * 4);
        let node_type = match self.dom_tree[element_idx].ty {
            ElementType::Layout => "Layout",
            ElementType::Label(_) => "Label",
            ElementType::TextEdit(_) => "TextEdit"
        };

        println!(
            "{}{} ({})",
            indent,
            self.dom_tree[element_idx].uuid(),
            node_type,
        );

        for &child_idx in &self.dom_tree[element_idx].children {
            self._traverse_and_print(child_idx, level + 1);
        }
    }

    pub fn generate_draw_calls(&mut self) -> Vec<Primitive> {
        self._generate_draw_calls(self.root_idx)
    }

    fn _generate_draw_calls(&mut self, element_idx: usize) -> Vec<Primitive> {
        let mut draw_calls = self.dom_tree[element_idx].draw();

        let children_ids: Vec<usize> = self.dom_tree[element_idx].children.clone();

        for &child_idx in children_ids.iter() {
            draw_calls.extend(self._generate_draw_calls(child_idx));
        }

        draw_calls
    }

    pub fn with_style_sheet(&mut self, styles: StyleSheet) -> &mut Self {
        self.style_stack.push((None, styles));
        self
    }
}
