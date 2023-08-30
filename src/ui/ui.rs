use crate::element::{Element, ElementType};
use crate::style::StyleSheet;
use std::cell::{RefCell};
use std::collections::VecDeque;
use std::rc::Rc;
use uuid::Uuid;
use crate::paint::{Drawable, Primitive};

pub struct Ui {
    pub root: Rc<RefCell<Element>>,
    pub current_parent: VecDeque<Rc<RefCell<Element>>>,
    pub style_stack: Vec<(Option<Uuid>, StyleSheet)>,
}

impl Default for Ui {
    fn default() -> Self {
        let root = Element::new("root".to_string(), ElementType::Layout, None);
        Ui {
            root: root.clone(),
            current_parent: vec![root].into(),
            style_stack: Vec::new(),
        }
    }
}

impl Ui {
    pub fn create_node(&mut self, new_node: Rc<RefCell<Element>>) {
        for (owner_opt, sheet) in self.style_stack.iter_mut() {
            if let Some(_) = owner_opt {
                new_node.borrow_mut().style.inherit(sheet);
            } else {
                *owner_opt = Some(new_node.borrow().uuid());
                new_node.borrow_mut().style.apply(sheet);
            }
        }

        self.current_parent
            .front_mut()
            .unwrap()
            .borrow_mut()
            .add_child(new_node.clone());
    }

    pub fn unbind_styles(&mut self, owner_ref: Rc<RefCell<Element>>) {
        if !self.style_stack.is_empty() {
            if let Some(uuid) = self.style_stack.last().unwrap().0 {
                if owner_ref.borrow().uuid() == uuid {
                    self.style_stack.pop();
                }
            }
        }
    }

    pub fn traverse_and_print(&self) {
        Ui::_traverse_and_print(&self.root, 0);
    }

    fn _traverse_and_print(node: &Rc<RefCell<Element>>, level: usize) {
        let indent = " ".repeat(level * 4);
        let id = node.borrow().id.clone();
        let node_type = match node.borrow().ty {
            ElementType::Layout => "Layout",
            ElementType::Label => "Label",
            ElementType::Button => "Button",
        };

        println!(
            "{}{} ({} with uuid {})",
            indent,
            id,
            node_type,
            node.borrow().uuid(),
        );

        for child in &node.borrow().children {
            Ui::_traverse_and_print(child, level + 1);
        }
    }

    pub fn generate_draw_calls(&mut self) -> Vec<Primitive> {
        Ui::_generate_draw_calls(self.root.clone())
    }

    fn _generate_draw_calls(element: Rc<RefCell<Element>>) -> Vec<Primitive> {
        let mut draw_calls = element.borrow().draw();

        for child in element.borrow().children.iter() {
            draw_calls.extend(Ui::_generate_draw_calls(child.clone()));
        }

        draw_calls
    }

    pub fn with_style_sheet(&mut self, styles: StyleSheet) -> &mut Self {
        self.style_stack.push((None, styles));
        self
    }
}
