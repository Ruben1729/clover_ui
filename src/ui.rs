use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use uuid::Uuid;
use crate::element::{Element, ElementType};
use crate::style::{StyleSheet};

pub struct Ui {
    pub root: Rc<RefCell<Element>>,
    current_parent: VecDeque<Rc<RefCell<Element>>>,
    style_stack: Vec<(Option<Uuid>, StyleSheet)>
}

impl Default for Ui {
    fn default() -> Self {
        let root = Element::new("root".to_string(), ElementType::FlexBox, None);
        Ui {
            root: root.clone(),
            current_parent: vec![root].into(),
            style_stack: Vec::new()
        }
    }
}

impl Ui {
    pub fn flex(&mut self, add_contents: impl FnOnce(&mut Ui)) -> Rc<RefCell<Element>> {
        let new_flex_node = Element::new("flex".to_string(), ElementType::FlexBox, Some(self.current_parent.front().unwrap().clone()));

        if !self.style_stack.is_empty() {
            // We set the owner of this style to the first element that claims it
            if self.style_stack.last().unwrap().0 == None {
                self.style_stack.last_mut().unwrap().0 = Some(new_flex_node.borrow().uuid());
            }

            new_flex_node.borrow_mut().style.apply(&self.style_stack.last().unwrap().1);
        }

        self.current_parent.front_mut().unwrap().borrow_mut().add_child(new_flex_node.clone());
        self.current_parent.push_front(new_flex_node.clone());

        add_contents(self);

        if !self.style_stack.is_empty() {
            if let Some(uuid) = self.style_stack.last().unwrap().0 {
                if new_flex_node.borrow().uuid() == uuid {
                    self.style_stack.pop();
                }
            }
        }

        self.current_parent.pop_front();
        new_flex_node
    }

    pub fn button(&mut self, id: String) -> Rc<RefCell<Element>> {
        let new_node = Element::new(id, ElementType::Button, Some(self.current_parent.front().unwrap().clone()));

        if !self.style_stack.is_empty() {
            new_node.borrow_mut().style.apply(&self.style_stack.last().unwrap().1);
        }

        self.current_parent.front_mut().unwrap().borrow_mut().add_child(new_node.clone());
        new_node
    }

    pub fn label(&mut self, id: String) -> Rc<RefCell<Element>> {
        let new_node = Element::new(id, ElementType::Label, Some(self.current_parent.front().unwrap().clone()));

        if !self.style_stack.is_empty() {
            new_node.borrow_mut().style.apply(&self.style_stack.last().unwrap().1);
        }

        self.current_parent.front_mut().unwrap().borrow_mut().add_child(new_node.clone());
        new_node
    }

    pub fn traverse_and_print(&self) {
        Ui::_traverse_and_print(&self.root, 0);
    }

    fn _traverse_and_print(node: &Rc<RefCell<Element>>, level: usize) {
        let indent = " ".repeat(level * 4);
        let id = node.borrow().id.clone();
        let node_type = match node.borrow().ty {
            ElementType::FlexBox => "Flex",
            ElementType::Label => "Label",
            ElementType::Button => "Button"
        };

        println!("{}{} ({} with bg {})", indent, id, node_type, node.borrow().style.get_backgroundcolor().get_u32());

        for child in &node.borrow().children {
            Ui::_traverse_and_print(child, level + 1);
        }
    }

    pub fn with_style_sheet(&mut self, styles: StyleSheet) -> &mut Self {
        self.style_stack.push((None, styles));
        self
    }
}