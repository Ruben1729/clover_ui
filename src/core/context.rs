use bytemuck::{Pod, Zeroable};
use crate::core::render::Primitive;
use crate::styles::{Style};
use crate::styles::preset::color::COLOR_TRANSPARENT;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: u32,
}

#[derive(Default, Debug)]
pub struct Cursor {
    pub x: isize,
    pub y: isize
}

#[derive(Default, Debug)]
pub struct Context {
    pub cursor: Cursor,

    pub render_calls: Vec<Primitive>
}

impl Context {
    pub fn get_vertex_data(&self) -> Vec<Vertex> {
        let mut vertex_data = vec![];

        for &call in self.render_calls.iter() {
            match call {
                Primitive::Circle { .. } => {}
                Primitive::Rectangle {
                    x,
                    y,
                    width,
                    height,
                    color
                } => {
                    vertex_data.push(Vertex { position: [x, y], color });
                    vertex_data.push(Vertex { position: [x + width, y], color });
                    vertex_data.push(Vertex { position: [x + width, y + height], color });
                    vertex_data.push(Vertex { position: [x, y], color });
                    vertex_data.push(Vertex { position: [x + width, y + height], color });
                    vertex_data.push(Vertex { position: [x, y + height], color });
                }
            }
        }

        vertex_data
    }
    pub fn draw_box(&mut self, style: &Style) {
        let box_model = &style.box_model;
        // draw content box with padding
        if (box_model.padding.horizontal() + box_model.content.width) > 0 && (box_model.padding.vertical() + box_model.content.height) > 0 {
            self.render_calls.push(Primitive::Rectangle {
                x: (box_model.x + box_model.margin.left + box_model.border.left) as f32,
                y: (box_model.y + box_model.margin.top + box_model.border.top) as f32,
                width: (box_model.padding.horizontal() + box_model.content.width) as f32,
                height: (box_model.padding.vertical() + box_model.content.height) as f32,
                color: style.background_color.get_u32(),
            });
        }

        // draw border box
        if box_model.has_border() {
            self.render_calls.push(Primitive::Rectangle {
                x: (box_model.x + box_model.margin.left) as f32,
                y: (box_model.y + box_model.margin.top) as f32,
                width: (box_model.border.horizontal() + box_model.padding.horizontal() + box_model.content.width) as f32,
                height: (box_model.border.vertical() + box_model.padding.vertical() + box_model.content.height) as f32,
                color: style.border_color.get_u32(),
            });
        }

        // draw margin box
        if box_model.has_margin() {
            self.render_calls.push(Primitive::Rectangle {
                x: box_model.x as f32,
                y: box_model.y as f32,
                width: box_model.width() as f32,
                height: box_model.height() as f32,
                color: COLOR_TRANSPARENT.get_u32(),
            });
        }
    }
}
