use crate::element::{Element, ElementType};
use crate::paint::Drawable;
use crate::style::{COLOR_WHITE, FlexDirection, Layout};
use crate::ui::Ui;

impl Ui {
    pub fn page(&mut self, add_contents: impl FnOnce(&mut Ui)) {
        let layout = Layout {
            flex_direction: FlexDirection:: Col
        };
        let mut new_element = Element::new(ElementType::Container(layout.clone()));

        // TODO: Style with some sort of theme system
        new_element.style.set_x(0);
        new_element.style.set_y(0);
        new_element.style.set_width(1280);
        new_element.style.set_height(720);
        new_element.style.set_backgroundcolor(COLOR_WHITE);

        self.create_layout(&mut new_element, add_contents);
    }

    // 1- Style
    // 2- Handle Event
    // 3- Add Content
    pub fn create_layout(&mut self, new_element: &mut Element, add_contents: impl FnOnce(&mut Ui)) {
        let layout;
        if let ElementType::Container(elem_layout) = &new_element.ty {
            layout = elem_layout.clone();
        } else {
            panic!("You tried making a layout on an element that isn't a layout.")
        }

        // Style and handle events
        self.inherit_style(new_element);
        self.dispatch_events(new_element);

        // Update cursor
        self.move_to_cursor(new_element);
        self.move_cursor_to_content(new_element);

        // Draw before adding children so children get rendered next
        self.draw_calls.extend(new_element.draw());

        // Set the layout for the children
        self.parent_layout.push_front(layout);
        add_contents(self);
        self.parent_layout.pop_front();

        self.move_cursor_to_next_element(&new_element);
    }
    pub fn flex_col(&mut self, add_contents: impl FnOnce(&mut Ui)) {
        let mut new_element = Element::new(ElementType::Container(Layout {
            flex_direction: FlexDirection::Col
        }));
        // TODO: we unbind here so that the page (which is the base) doesn't have their styles unbinded as
        // all it's children should inherit from it.
        self.create_layout(&mut new_element, add_contents);
        self.unbind_styles(new_element.uuid());
    }

    pub fn flex_row(&mut self, add_contents: impl FnOnce(&mut Ui)) {
        let mut new_element = Element::new(ElementType::Container(Layout {
            flex_direction: FlexDirection::Row
        }));

        self.create_layout(&mut new_element, add_contents);
        self.unbind_styles(new_element.uuid());
    }

    pub fn flex_row_reverse(&mut self, add_contents: impl FnOnce(&mut Ui)) {
        let mut new_element = Element::new(ElementType::Container(Layout {
            flex_direction: FlexDirection::RowReverse
        }));

        self.create_layout(&mut new_element, add_contents);
        self.unbind_styles(new_element.uuid());
    }

    pub fn flex_col_reverse(&mut self, add_contents: impl FnOnce(&mut Ui)) {
        let mut new_element = Element::new(ElementType::Container(Layout {
            flex_direction: FlexDirection::ColReverse
        }));

        self.create_layout(&mut new_element, add_contents);
        self.unbind_styles(new_element.uuid());
    }
}