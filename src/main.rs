extern crate minifb;

use std::path::Path;
use minifb::{Key, MouseMode, ScaleMode, Window, WindowOptions};
use clover_ui::component::{compute_positions, compute_dimensions, traverse};
use clover_ui::element::{Element, ElementBuilder, ElementType};
use clover_ui::context::Context;
use clover_ui::paint::{Drawable, DrawingBackend, Painter};
use clover_ui::style::{Border, Color, FlexDirection, FontManager, Spacing, StyleProperty};
use clover_ui::style::Display::Flex;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

pub struct MiniFbBackend {
    buffer: Vec<u32>,
}

impl MiniFbBackend {
    pub fn new() -> Self {
        Self {
            buffer: vec![0; WIDTH * HEIGHT],
        }
    }
}

impl DrawingBackend for MiniFbBackend {
    fn draw_circle(&mut self, x: f32, y: f32, radius: f32, color: u32) {
        // Implement circle drawing algorithm here
        // Update self.buffer with circle pixels
    }

    fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32, color: u32) {
        for i in 0..(width as i32) {
            for j in 0..(height as i32) {
                let x_pixel = (x as i32 + i) as usize;
                let y_pixel = (y as i32 + j) as usize;

                if x_pixel < WIDTH && y_pixel < HEIGHT {
                    self.buffer[y_pixel * WIDTH + x_pixel] = color;
                }
            }
        }
    }

    fn draw_pixel(&mut self, x: f32, y: f32, color: u32) {
        let x = x as usize;
        let y = y as usize;

        if x < WIDTH && y < HEIGHT {
            self.buffer[y * WIDTH + x] = color;
        }
    }

    fn draw_pixel_with_blending(&mut self, x: f32, y: f32, color: u32, v: f32) {
        let x = x as usize;
        let y = y as usize;

        if x < WIDTH && y < HEIGHT {
            let pixel = self.buffer.get_mut((x as usize) + (y as usize) * WIDTH).unwrap();

            // Separate the channels of the existing color in the buffer
            let alpha_old = (*pixel >> 24) & 0xFF;
            let red_old   = (*pixel >> 16) & 0xFF;
            let green_old = (*pixel >> 8)  & 0xFF;
            let blue_old  = (*pixel)       & 0xFF;

            // Separate the channels of the new color you want to blend
            let alpha_new = ((color >> 24) & 0xFF) as f32 * v;
            let red_new   = ((color >> 16) & 0xFF) as f32 * v;
            let green_new = ((color >> 8)  & 0xFF) as f32 * v;
            let blue_new  = ((color)       & 0xFF) as f32 * v;

            // Perform alpha blending for each channel
            let alpha = (alpha_old as f32 * (1.0 - v) + alpha_new) as u32;
            let red   = (red_old   as f32 * (1.0 - v) + red_new)   as u32;
            let green = (green_old as f32 * (1.0 - v) + green_new) as u32;
            let blue  = (blue_old  as f32 * (1.0 - v) + blue_new)  as u32;

            let color = (alpha << 24) | (red << 16) | (green << 8) | blue;

            *pixel = color;
        }
    }
}

fn main() {
    {
        let mut font_manager = FontManager::get_mut();
        font_manager.load(None,Path::new("/run/media/rubens/ssd/projects/clover_ui/inter.ttf")).expect("Unable to load font");
    }
    let mut backend = MiniFbBackend::new();

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

    let mut context = Context::default();

    let root = ElementBuilder::default()
        .with_id("root".to_string())
        .with_styles(
            vec![
                StyleProperty::Border(Border::new(5, Color::default())),
                StyleProperty::Height(HEIGHT),
                StyleProperty::Width(WIDTH)
            ]
        )
        .with_style_on_hover(
            vec![
                StyleProperty::Border(Border::new(5, Color::new(255, 255, 255, 255)))
            ]
        )
        .build();

    let flex_parent = ElementBuilder::default()
        .with_id("flex_parent".to_string())
        .with_styles(
            vec![
                StyleProperty::Margin(Spacing::new(10)),
                StyleProperty::Border(Border::new(10, Color::new(255, 0, 0, 255))),
                StyleProperty::Display(Flex {
                    direction: FlexDirection::Col,
                })
            ]
        )
        .build();

    let red_child = ElementBuilder::default()
        .with_id("red_child".to_string())
        .with_styles(
            vec![
                StyleProperty::Margin(Spacing::new(10)),
                StyleProperty::Padding(Spacing::new(10)),
                StyleProperty::BackgroundColor(Color::new(255, 255, 0, 0))
            ]
        ).build();

    let green_child = ElementBuilder::default()
        .with_id("green_child".to_string())
        .with_styles(
            vec![
                StyleProperty::Margin(Spacing::new(10)),
                StyleProperty::Padding(Spacing::new(10)),
                StyleProperty::BackgroundColor(Color::new(255, 0, 255, 0)),
                StyleProperty::Color(Color::new(255, 255, 255, 255)),
                StyleProperty::Display(Flex {
                    direction: FlexDirection::Col,
                })
            ]
        ).build();

    let blue_child = ElementBuilder::default()
        .with_id("blue_child".to_string())
        .with_styles(
            vec![
                StyleProperty::Padding(Spacing::new(10)),
                StyleProperty::BackgroundColor(Color::new(255, 0, 0, 255)),
                StyleProperty::Border(Border::new(10, Color::default()))
            ]
        ).with_style_on_hover(
            vec![
                StyleProperty::Border(Border::new(10, Color::new(255, 255, 255 ,255))),
                StyleProperty::BackgroundColor(Color::new(100, 150, 150, 150))
            ]
        )
        .build();

    let custom_child = ElementBuilder::new(ElementType::Label {
        value: "Testing".to_string()
    })
        .with_id("custom".to_string())
        .with_styles(
            vec![
                StyleProperty::BackgroundColor(Color::new(255, 255, 0, 255)),
                StyleProperty::FontSize(40.0),
                StyleProperty::Color(Color::new(255, 255, 255, 255)),
                StyleProperty::Border(Border::new(5, Color::new(255, 255, 0, 255)))
            ]
        )
        .with_style_on_hover(
            vec![
                StyleProperty::Border(Border::new(5, Color::new(255, 255, 255, 255)))
            ]
        ).build();

    Element::insert(&root, &flex_parent);
    Element::insert(&flex_parent, &red_child);
    Element::insert(&flex_parent, &green_child);
    Element::insert(&green_child, &custom_child);
    Element::insert(&green_child, &blue_child);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.get_mouse_down(minifb::MouseButton::Left) {
            context.set_mouse_pressed(clover_ui::state::MouseButton::Left);
        }
        if window.get_mouse_down(minifb::MouseButton::Right) {
            context.set_mouse_pressed(clover_ui::state::MouseButton::Right);
        }
        if window.get_mouse_down(minifb::MouseButton::Middle) {
            context.set_mouse_pressed(clover_ui::state::MouseButton::Middle);
        }

        context.set_mouse_pos(window.get_unscaled_mouse_pos(MouseMode::Discard));
        context.set_mouse_scroll(window.get_scroll_wheel());

        // Updates the state and generates events
        context.next();
        {
            compute_dimensions(root.clone());
            compute_positions(root.clone(), 0, 0);
            while !context.event_queue.is_empty() {
                let event = context.event_queue.remove(0);
                context.dispatch_event(root.clone(), &event);
            }
        }

        {
            let mut painter = Painter::new(&mut backend);
            traverse(&root, |elem| {
                painter.extend(elem.draw());
            });
            painter.draw();
        }
        // traverse(&root, |elem| {
        //     let style = elem.style();
        //
        //     for dy in 0..style.height() {
        //         for dx in 0..style.width() {
        //             let index = (style.x + dx) + (style.y + dy) * WIDTH;
        //             if let Some(color) = style.color_at_px(dx, dy) {
        //                 buffer[index] = color;
        //             }
        //         }
        //     }
        // });

        window.update_with_buffer(&backend.buffer, WIDTH, HEIGHT).unwrap();
    }
}
