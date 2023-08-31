use crate::event::Event;
use crate::ui::Ui;

impl<'a> Ui<'a> {
    pub fn dispatch_event(&mut self, event: &Event) {
        self._dispatch_event(self.root_idx, event);
    }
    pub fn _dispatch_event(&mut self, element_idx: usize, event: &Event) {
        {
            let children_ids = self.dom_tree[element_idx].children.clone();
            for &child_idx in children_ids.iter() {
                self._dispatch_event(child_idx, event);
            }
        }

        let initial_hovered = self.dom_tree[element_idx].is_hovered();

        self.dom_tree[element_idx].handle_event(event);

        let is_hovered = self.dom_tree[element_idx].is_hovered();

        if is_hovered != initial_hovered {
            let mut new_event = Event::MouseEnter;
            if !is_hovered {
                new_event = Event::MouseLeave;
            }
            self.dom_tree[element_idx].handle_event(&new_event);
        }
    }
}
