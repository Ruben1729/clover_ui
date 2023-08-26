use std::cell::RefCell;
use std::rc::Rc;
use crate::layout::Layout;

#[derive(Debug)]
pub enum ElementType {
    Div
}

pub struct Element {
    ty:             ElementType,
    id:             String,
    class:          Vec<String>,
    pub layout:     Layout,

    parent:         Option<Rc<RefCell<Element>>>,
    pub children:   Vec<Rc<RefCell<Element>>>,
}

impl Element {
    pub fn new(ty: ElementType) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Element {
            ty,
            id:         Default::default(),
            class:      Default::default(),
            layout:     Default::default(),
            parent:     Default::default(),
            children:   Default::default(),
        }))
    }

    pub fn insert(parent: &Rc<RefCell<Self>>, child: &Rc<RefCell<Self>>) {
        child.borrow_mut().parent = Some(Rc::clone(parent));
        parent.borrow_mut().children.push(Rc::clone(child));
    }
}
