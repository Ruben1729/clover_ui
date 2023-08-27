extern crate minifb;

use minifb::{Key, ScaleMode, Window, WindowOptions};
use clover_ui::component::{compute_positions, compute_dimensions, traverse};
use clover_ui::element::{Element, ElementBuilder};
use clover_ui::layout::{Color, Display, FlexDirection, FlexProperties, LayoutBuilder};
use clover_ui::layout::Display::Flex;

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

    let root =
        ElementBuilder::default()
        .with_layout(
        LayoutBuilder::default()
            .with_border_color(Color::new(255, 255, 255, 255))
            .with_border_width(5)
            .with_height(HEIGHT)
            .with_width(WIDTH)
            .build()
        ).build();

    let flex_parent = ElementBuilder::default()
        .with_layout(
            LayoutBuilder::default()
                .with_margin(10, 10, 10, 10)
                .with_border_width(10)
                .with_border_color(Color::new(255, 0, 0, 255))
                .with_display(Flex {
                    direction: FlexDirection::Col
                })
                .build()
        )
        .build();

    let red_child = ElementBuilder::default()
        .with_layout(
            LayoutBuilder::default()
                .with_margin(10, 10, 10, 10)
                .with_padding(10, 10, 10, 10)
                .with_background_color(Color::new(255, 255, 0, 0))
                .build()
        ).build();

    let green_child = ElementBuilder::default()
        .with_layout(
            LayoutBuilder::default()
                .with_margin(10, 10, 10, 10)
                .with_padding(10, 10, 10, 10)
                .with_background_color(Color::new(255, 0, 255, 0))
                .with_display(Flex {
                    direction: FlexDirection::Col
                })
                .build()
        ).build();

    let blue_child = ElementBuilder::default()
        .with_layout(
            LayoutBuilder::default()
                .with_margin(10, 10, 10, 10)
                .with_padding(10, 10, 10, 10)
                .with_background_color(Color::new(255, 0, 0, 255))
                .build()
        ).build();

    let custom_child = ElementBuilder::default()
        .with_layout(
            LayoutBuilder::default()
                .with_margin(10, 10, 10, 10)
                .with_padding(10, 10, 10, 10)
                .with_background_color(Color::new(255, 255, 0, 255))
                .build()
        ).build();

    Element::insert(&root, &flex_parent);
    Element::insert(&flex_parent, &red_child);
    Element::insert(&flex_parent, &blue_child);
    Element::insert(&flex_parent, &green_child);
    Element::insert(&green_child, &custom_child);

    while window.is_open() && !window.is_key_down(Key::Escape) {

        {
            compute_dimensions(root.clone());
            compute_positions(root.clone(), 0, 0);
        }

        traverse(&root, |elem| {
            for dy in 0..elem.layout.height() {
                for dx in 0..elem.layout.width() {
                    let index = (elem.layout.x + dx) + (elem.layout.y + dy) * WIDTH;
                    buffer[index] = elem.layout.color_at_px(dx, dy);
                }
            }
        });

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
