use std::cell::RefCell;
use std::rc::Rc;
use crate::element::{Element, ElementType};
use crate::style::{Layout, FlexDirection};
use crate::ui::Ui;

impl Ui {
    pub fn compute_dimensions(&mut self, element_ref: Rc<RefCell<Element>>) {
        let mut content_w = 0;
        let mut content_h = 0;

        let mut element = element_ref.borrow_mut();

        match element.ty {
            ElementType::Layout => {
                (content_w, content_h) = match &element.style.get_display() {
                    Layout::Block => {
                        (element.children.iter()
                             .map(|child| child.borrow().style.get_width())
                             .max().unwrap_or_default(),
                         element.children.iter()
                             .map(|child| child.borrow().style.get_height())
                             .sum())
                    }
                    Layout::Flex {
                        flex_direction,
                        ..
                        } => {
                        match flex_direction {
                            FlexDirection::Row | FlexDirection::RowReverse => {
                                (element.children.iter()
                                     .map(|child| child.borrow().style.get_width())
                                     .sum(),
                                 element.children.iter()
                                     .map(|child| child.borrow().style.get_height())
                                     .max().unwrap_or_default())
                            }
                            FlexDirection::Col | FlexDirection::ColReverse => {
                                (element.children.iter()
                                     .map(|child| child.borrow().style.get_width())
                                     .max().unwrap_or_default(),
                                 element.children.iter()
                                     .map(|child| child.borrow().style.get_height())
                                     .sum())
                            }
                        }
                    }
                    Layout::Grid { .. } |
                    Layout::InlineBlock |
                    Layout::Hidden => {
                        (0,0)
                    },
                };
            }
            ElementType::Label => {}
            ElementType::Button => {}
        }

        element.style.set_width(content_w);
        element.style.set_height(content_h);
    }
}