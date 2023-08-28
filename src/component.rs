use std::cell::RefCell;
use std::rc::Rc;
use crate::element::Element;
use crate::layout::{Display, FlexDirection};

pub fn traverse<F>(root: &Rc<RefCell<Element>>, mut action: F)
    where
        F: FnMut(&Element),
{
    let mut stack = Vec::new();
    stack.push(Rc::clone(root));

    while let Some(current) = stack.pop() {
        let current_borrow = current.borrow();
        action(&*current_borrow);

        for child in current_borrow.children.iter().rev() {
            stack.push(Rc::clone(child));
        }
    }
}

pub fn compute_dimensions(parent_ref: Rc<RefCell<Element>>) {
    let mut parent = parent_ref.borrow_mut();
    for child in &parent.children {
        compute_dimensions(child.clone());
    }

    let (mut new_width, mut new_height) = match &parent.layout.display {
        Display::Block() => {
            (parent.children.iter()
                 .map(|child| child.borrow().layout.width())
                 .max().unwrap_or_default(),
             parent.children.iter()
                 .map(|child| child.borrow().layout.height())
                 .sum())
        }
        Display::Flex{ direction } => {
            match direction {
                FlexDirection::Row | FlexDirection::RowReverse => {
                    (parent.children.iter()
                         .map(|child| child.borrow().layout.width())
                         .sum(),
                     parent.children.iter()
                         .map(|child| child.borrow().layout.height())
                         .max().unwrap_or_default())
                }
                FlexDirection::Col | FlexDirection::ColReverse => {
                    (parent.children.iter()
                         .map(|child| child.borrow().layout.width())
                         .max().unwrap_or_default(),
                     parent.children.iter()
                         .map(|child| child.borrow().layout.height())
                         .sum())
                }
            }
        }
    };

    // Calculate width
    parent.layout.width = new_width;

    // Calculate height
    parent.layout.height = new_height;
}

pub fn compute_positions(parent_ref: Rc<RefCell<Element>>, dx: usize, dy: usize) {
    {
        let mut parent = parent_ref.borrow_mut();
        parent.layout.x = dx;
        parent.layout.y = dy;
    }

    // The borrow of elem has been dropped at this point, so we can borrow it again in the recursion.
    let parent = parent_ref.borrow();

    let (mut new_dx, mut new_dy) = match &parent.layout.display {
        Display::Block() => {
            (parent.layout.content_x(), parent.layout.content_y())
        }
        Display::Flex{ direction } => {
            match direction {
                FlexDirection::Row | FlexDirection::Col => {
                    (parent.layout.content_x(), parent.layout.content_y())
                }
                FlexDirection::RowReverse => {
                    (parent.layout.content_x() + parent.layout.width, parent.layout.content_y())
                }
                FlexDirection::ColReverse => {
                    (parent.layout.content_x(), parent.layout.content_y() + parent.layout.height)
                }
            }
        }
    };

    for child in &parent.children {
        {
            match &parent.layout.display {
                Display::Block() => {}
                Display::Flex{direction} => {
                    match direction {
                        FlexDirection::Row | FlexDirection::Col => { }
                        FlexDirection::RowReverse => {
                            new_dx -= child.borrow().layout.width();
                        }
                        FlexDirection::ColReverse => {
                            new_dy -= child.borrow().layout.height();
                        }
                    }
                }
            }
        }

        compute_positions(child.clone(), new_dx, new_dy);

        {
            match &parent.layout.display {
                Display::Block() => {
                    new_dy += child.borrow().layout.height();
                }
                Display::Flex { direction } => {
                    match direction {
                        FlexDirection::Row => {
                            new_dx += child.borrow().layout.width();
                        }
                        FlexDirection::RowReverse | FlexDirection::ColReverse=> { }
                        FlexDirection::Col => {
                            new_dy += child.borrow().layout.height();
                        }
                    }
                }
            }
        }
    }
}
