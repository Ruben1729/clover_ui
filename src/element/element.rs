use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;
use crate::style::{StyleSheet};

pub struct Element {
    pub id: String,
    uuid: Uuid,
    pub ty: ElementType,
    pub style: StyleSheet,

    parent: Option<Rc<RefCell<Element>>>,
    pub children: Vec<Rc<RefCell<Element>>>,
}

pub enum ElementType {
    FlexBox,
    Label,
    Button
}

impl Element {
    pub fn new(id: String, ty: ElementType, parent: Option<Rc<RefCell<Element>>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            id,
            uuid: Uuid::new_v4(),
            ty,
            style: StyleSheet::new(),
            parent,
            children: vec![],
        }))
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Element>>) {
        self.children.push(child);
    }
}
