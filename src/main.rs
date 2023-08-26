extern crate minifb;

use std::cell::RefCell;
use std::rc::Rc;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use clover_ui::element::{Element, ElementType};
use clover_ui::layout::Color;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn draw_rectangle(buffer: &mut [u32], x: usize, y: usize, width: usize, height: usize, color: u32) {
    for dy in 0..height {
        for dx in 0..width {
            let index = (x + dx) + (y + dy) * WIDTH;
            buffer[index] = color;
        }
    }
}


fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale_mode: ScaleMode::UpperLeft,
            ..Default::default()
        },
    )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    let root = Element::new(ElementType::Div);
    {
        let mut root_el = root.borrow_mut();
        root_el.layout.width = WIDTH;
        root_el.layout.height = HEIGHT;
    }


    let child = Element::new(ElementType::Div);
    {
        let mut child_el = child.borrow_mut();
        child_el.layout.width = 100;
        child_el.layout.height = 100;
        child_el.layout.background_color = Color::new(255, 255, 0, 0);
    }

    Element::insert(&root, &child);

    while window.is_open() && !window.is_key_down(Key::Escape) {

        traverse(&root, |elem| {
            // Execute logic for each element immediately
            draw_rectangle(&mut buffer,
                           elem.layout.x,
                           elem.layout.y,
                           elem.layout.width,
                           elem.layout.height,
                           elem.layout.background_color.get_u32());
        });

        // Update the window with the buffer data
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn traverse<F>(root: &Rc<RefCell<Element>>, mut action: F)
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


