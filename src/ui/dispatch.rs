use std::cell::RefCell;
use std::rc::Rc;
use crate::element::Element;
use crate::event::Event;
use crate::ui::Ui;

impl Ui {
    pub fn dispatch_event(&mut self, event: &Event) {
        Ui::_dispatch_event(self.root.clone(), event);
    }
    pub fn _dispatch_event(element_ref: Rc<RefCell<Element>>, event: &Event) {
        {
            let parent = element_ref.borrow();
            for child in &parent.children {
                Ui::_dispatch_event(child.clone(), event);
            }
        }

        let mut initial_hovered = false;
        let mut is_hovered = false;

        {
            let element = element_ref.borrow();
            initial_hovered = element.is_hovered()
        }

        {
            let mut element = element_ref.borrow_mut();
            element.handle_event(event);
        }

        {
            let element = element_ref.borrow();
            is_hovered = element.is_hovered()
        }

        if is_hovered != initial_hovered {
            let mut new_event = Event::MouseEnter;
            if !is_hovered {
                new_event = Event::MouseLeave;
            }

            {
                let mut element = element_ref.borrow_mut();
                element.handle_event(&new_event);
            }
        }
    }
}
