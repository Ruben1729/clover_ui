use crate::element::{Element, ElementState, ElementStateManager, ElementType};
use crate::style::{FontManager};
use crate::ui::Ui;
use rusttype::{point, Scale};
use crate::event::Event;

impl Ui {
    pub fn dispatch_events(&mut self, new_element: &mut Element) {
        self.running_counter += 1;
        let mut element_state = self.persistent_element_state.entry(self.running_counter).or_insert_with(ElementStateManager::default);

        for event in &self.context.event_queue {
            match event {
                Event::KeyDown(_) => {}
                Event::KeyUp(_) => {}
                Event::MouseScroll(_) => {}
                Event::MouseMove(state) => {
                    if let Some((x, y)) = state.pos {
                        if new_element.cursor_in_bounds(x, y) {
                            element_state.push(ElementState::Hovered);
                        } else {
                            element_state.remove(ElementState::Hovered);
                        }
                    } else {
                        element_state.remove(ElementState::Hovered);
                    }
                }
                Event::MouseDown { state, .. } => {
                    if let Some(position) = state.pos {
                        if new_element.cursor_in_bounds(position.0, position.1) {
                            element_state.push(ElementState::MouseDown);
                        }
                    }
                }
                Event::MouseUp { state, button } => {
                    if let Some(position) = state.pos {
                        if new_element.cursor_in_bounds(position.0, position.1) {
                            element_state.push(ElementState::Clicked);
                        }
                    }
                }
            }
        }

        for new_state in &element_state.states {
            new_element.state_manager.push(new_state.clone());
        }
    }
    pub fn compute_text_dimensions(&self, element: &mut Element) {
        if let ElementType::Label(value) = element.ty {
            let manager = FontManager::get();
            let font = manager
                .get_font(None, element.style.get_fontweight())
                .expect("Unable to load font");

            let scale = Scale::uniform(element.style.get_fontsize());
            let v_metrics = font.v_metrics(scale);

            let glyphs: Vec<_> = font
                .layout(value, scale, point(0.0, 0.0 + v_metrics.ascent))
                .collect();

            let (content_w, content_h) = (
                {
                    let min_x = glyphs
                        .first()
                        .map(|g| g.pixel_bounding_box().unwrap().min.x)
                        .unwrap();
                    let max_x = glyphs
                        .last()
                        .map(|g| g.pixel_bounding_box().unwrap().max.x)
                        .unwrap();
                    (max_x - min_x) as usize
                },
                (v_metrics.ascent - v_metrics.descent).ceil() as usize,
            );

            element.style.set_width(content_w);
            element.style.set_height(content_h);
        } else {
            panic!("Attempted to call compute content dimensions on a non label.");
        }
    }
}
