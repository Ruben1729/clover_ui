use crate::element::Element;
use crate::events::Events;
use crate::state::{Key, MouseButton, State};
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

#[derive(Default)]
pub struct Context {
    prev_state: State,
    curr_state: State,

    pub event_queue: Vec<Events>,
}

impl Context {
    pub fn next(&mut self) {
        // We dont update the previous state to the current state throughout this function until the end.
        // Modifying the current state should be avoided in this function as the previous state and current state
        // are simply swapped.

        // KEY DOWN & UP EVENT
        for key in &self.curr_state.keys.pressed {
            self.event_queue.push(Events::KeyDown(key.clone()));
        }
        self.prev_state
            .keys
            .pressed
            .retain(|key| !self.curr_state.keys.pressed.contains(key));

        for key in &self.prev_state.keys.pressed {
            self.event_queue.push(Events::KeyUp(key.clone()));
        }

        // MOUSE DOWN & UP EVENT
        for button in &self.curr_state.mouse.pressed {
            self.event_queue.push(Events::MouseDown(button.clone()));
            if !self.prev_state.mouse.pressed.contains(button) {
                self.event_queue.push(Events::Click(button.clone()));
            }
        }
        self.prev_state
            .mouse
            .pressed
            .retain(|key| !self.curr_state.mouse.pressed.contains(key));

        for button in &self.prev_state.mouse.pressed {
            self.event_queue.push(Events::MouseUp(button.clone()));
        }

        // MOUSE MOVE
        if self.curr_state.mouse.pos != self.prev_state.mouse.pos {
            self.event_queue
                .push(Events::MouseMove(self.curr_state.mouse.pos));
        }

        // MOUSE SCROLL
        if self.curr_state.mouse.scroll_wheel != self.prev_state.mouse.scroll_wheel {
            self.event_queue
                .push(Events::MouseScroll(self.curr_state.mouse.scroll_wheel));
        }

        // SWAP STATES AND CLEAR CURRENT STATE TO USE FOR RECORDING
        mem::swap(&mut self.prev_state, &mut self.curr_state);
        self.curr_state.clear();
    }

    pub fn dispatch_event(&mut self, parent_ref: Rc<RefCell<Element>>, event: &Events) {
        {
            let parent = parent_ref.borrow();
            for child in &parent.children {
                self.dispatch_event(child.clone(), event);
            }
        }

        let mut initial_hovered = false;
        let mut is_hovered = false;

        {
            let parent = parent_ref.borrow();
            initial_hovered = parent.is_hovered()
        }

        {
            let mut parent = parent_ref.borrow_mut();
            parent.handle_event(event);
        }

        {
            let parent = parent_ref.borrow();
            is_hovered = parent.is_hovered()
        }

        if is_hovered != initial_hovered {
            let mut new_event = Events::MouseEnter;
            if !is_hovered {
                new_event = Events::MouseLeave;
            }

            {
                let mut parent = parent_ref.borrow_mut();
                parent.handle_event(&new_event);
            }
        }
    }
    pub fn set_mouse_pressed(&mut self, mouse: MouseButton) {
        self.curr_state.mouse.pressed.push(mouse);
    }

    pub fn set_mouse_pos(&mut self, pos: Option<(f32, f32)>) {
        self.curr_state.mouse.pos = pos
    }

    pub fn set_mouse_scroll(&mut self, scroll: Option<(f32, f32)>) {
        self.curr_state.mouse.scroll_wheel = scroll
    }

    pub fn set_key_pressed(&mut self, key: Key) {
        self.curr_state.keys.pressed.push(key);
    }
}
