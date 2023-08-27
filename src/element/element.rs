use std::cell::RefCell;
use std::rc::Rc;
use crate::layout::Layout;

pub enum ElementType {
    Text(String),
    Button,
}

pub struct Element {
    id:             String,
    class:          Vec<String>,
    pub layout:     Layout,

    parent:         Option<Rc<RefCell<Element>>>,
    pub children:   Vec<Rc<RefCell<Element>>>,
}

impl Element {
    pub fn new(id: String, class: Vec<String>, layout: Layout) -> Self {
        Element {
            id,
            class,
            layout,
            parent:     None,
            children:   Vec::new()
        }
    }
    pub fn insert(parent: &Rc<RefCell<Self>>, child: &Rc<RefCell<Self>>) {
        child.borrow_mut().parent = Some(Rc::clone(parent));
        parent.borrow_mut().children.push(Rc::clone(child));
    }
}
