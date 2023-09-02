use crate::event::Event;
use crate::state::{Key, MouseButton, State};
use std::mem;

#[derive(Default)]
pub struct Context {
    prev_state: State,
    curr_state: State,

    pub event_queue: Vec<Event>,
}

impl Context {
    pub fn next(&mut self) {
        // We dont update the previous state to the current state throughout this function until the end.
        // Modifying the current state should be avoided in this function as the previous state and current state
        // are simply swapped.

        // KEY DOWN & UP EVENT
        for key in &self.curr_state.keys.pressed {
            self.event_queue.push(Event::KeyDown(key.clone()));
        }
        self.prev_state
            .keys
            .pressed
            .retain(|key| !self.curr_state.keys.pressed.contains(key));

        for key in &self.prev_state.keys.pressed {
            self.event_queue.push(Event::KeyUp(key.clone()));
        }

        // MOUSE DOWN & UP EVENT
        for btn in &self.curr_state.mouse.pressed {
            self.event_queue.push(Event::MouseDown {
                state: self.curr_state.mouse.clone(),
                button: btn.clone(),
            });
        }
        self.prev_state
            .mouse
            .pressed
            .retain(|key| !self.curr_state.mouse.pressed.contains(key));

        for btn in &self.prev_state.mouse.pressed {
            self.event_queue.push(Event::MouseUp {
                state: self.curr_state.mouse.clone(),
                button: btn.clone(),
            });
        }

        // MOUSE MOVE
        if self.curr_state.mouse.pos != self.prev_state.mouse.pos {
            self.event_queue
                .push(Event::MouseMove(self.curr_state.mouse.clone()));
        }

        // MOUSE SCROLL
        if self.curr_state.mouse.scroll_wheel != self.prev_state.mouse.scroll_wheel {
            self.event_queue
                .push(Event::MouseScroll(self.curr_state.mouse.clone()));
        }

        // SWAP STATES AND CLEAR CURRENT STATE TO USE FOR RECORDING
        mem::swap(&mut self.prev_state, &mut self.curr_state);
        self.curr_state.clear();
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
