use crate::element::{Element, ElementType};
use crate::style::{FlexDirection, Layout};
use crate::ui::Ui;
use std::cell::RefCell;
use std::rc::Rc;

impl Ui {
    pub fn compute_dimensions(&mut self) {
        Ui::_compute_dimensions(self.root.clone());
    }
    fn _compute_dimensions(element_ref: Rc<RefCell<Element>>) {
        let mut element = element_ref.borrow_mut();

        for child in &element.children {
            Ui::_compute_dimensions(child.clone());
        }

        let mut content_w = 0;
        let mut content_h = 0;

        match element.ty {
            ElementType::Layout => {
                (content_w, content_h) = match &element.style.get_display() {
                    Layout::Block => (
                        element
                            .children
                            .iter()
                            .map(|child| child.borrow().style.get_total_width())
                            .max()
                            .unwrap_or_default(),
                        element
                            .children
                            .iter()
                            .map(|child| child.borrow().style.get_total_height())
                            .sum(),
                    ),
                    Layout::Flex { flex_direction, .. } => match flex_direction {
                        FlexDirection::Row | FlexDirection::RowReverse => (
                            element
                                .children
                                .iter()
                                .map(|child| child.borrow().style.get_total_width())
                                .sum(),
                            element
                                .children
                                .iter()
                                .map(|child| child.borrow().style.get_total_height())
                                .max()
                                .unwrap_or_default(),
                        ),
                        FlexDirection::Col | FlexDirection::ColReverse => (
                            element
                                .children
                                .iter()
                                .map(|child| child.borrow().style.get_total_width())
                                .max()
                                .unwrap_or_default(),
                            element
                                .children
                                .iter()
                                .map(|child| child.borrow().style.get_total_height())
                                .sum(),
                        ),
                    },
                    Layout::Grid { .. } | Layout::InlineBlock | Layout::Hidden => (0, 0),
                };
            }
            ElementType::Label => {}
            ElementType::Button => {}
        }

        element.style.set_width(content_w);
        element.style.set_height(content_h);
    }

    pub fn compute_positions(&mut self) {
        Ui::_compute_positions(self.root.clone(), 0, 0);
    }

    pub fn _compute_positions(element_ref: Rc<RefCell<Element>>, dx: usize, dy: usize) {
        {
            let mut element = element_ref.borrow_mut();
            element.style.set_x(dx);
            element.style.set_y(dy);
        }

        let element = element_ref.borrow();

        let (mut new_dx, mut new_dy) = match &element.style.get_display() {
            Layout::Block => (element.style.get_content_x(), element.style.get_content_y()),
            Layout::Flex { flex_direction, .. } => match flex_direction {
                FlexDirection::Row | FlexDirection::Col => {
                    (element.style.get_content_x(), element.style.get_content_y())
                }
                FlexDirection::RowReverse => (
                    element.style.get_content_x() + element.style.get_width(),
                    element.style.get_content_y(),
                ),
                FlexDirection::ColReverse => (
                    element.style.get_content_x(),
                    element.style.get_content_y() + element.style.get_height(),
                ),
            },
            _ => (0, 0),
        };
        for child in &element.children {
            {
                match &element.style.get_display() {
                    Layout::Flex { flex_direction, .. } => match flex_direction {
                        FlexDirection::Row | FlexDirection::Col => {}
                        FlexDirection::RowReverse => {
                            new_dx -= child.borrow().style.get_total_width();
                        }
                        FlexDirection::ColReverse => {
                            new_dy -= child.borrow().style.get_total_height();
                        }
                    },
                    _ => {}
                }
            }
            Ui::_compute_positions(child.clone(), new_dx, new_dy);
            {
                match &element.style.get_display() {
                    Layout::Block => {
                        new_dy += child.borrow().style.get_total_height();
                    }
                    Layout::Flex { flex_direction, .. } => match flex_direction {
                        FlexDirection::Row => {
                            new_dx += child.borrow().style.get_total_width();
                        }
                        FlexDirection::RowReverse | FlexDirection::ColReverse => {}
                        FlexDirection::Col => {
                            new_dy += child.borrow().style.get_total_height();
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}
