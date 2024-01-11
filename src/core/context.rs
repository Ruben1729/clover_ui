use std::iter::Filter;
use std::slice::Iter;
use bytemuck::{Pod, Zeroable};
use crate::core::render::Primitive;
use crate::styles::{Style};
use crate::styles::preset::color::COLOR_TRANSPARENT;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 4],
    pub color: [f32; 4],
}

#[derive(Default, Debug)]
pub struct Cursor {
    pub x: isize,
    pub y: isize
}

#[derive(Default, Debug)]
pub struct Context {
    pub cursor: Cursor,

    pub render_calls: Vec<Primitive>,
    pub current_depth: f32,
}

impl Context {
    pub fn get_text_data(&self) -> Filter<Iter<Primitive>, fn(&& Primitive) -> bool> {
        self.render_calls.iter().filter(|v| if let Primitive::Text { .. } = v { true } else { false })
    }
    pub fn get_vertex_data(&self) -> Vec<Vertex> {
        let mut vertex_data = vec![];

        for call in self.render_calls.iter() {
            match *call {
                Primitive::Circle { .. } => {}
                Primitive::Rectangle {
                    x,
                    y,
                    z,
                    width,
                    height,
                    color
                } => {
                    vertex_data.push(Vertex { position: [x, y, z, 1.0], color: color.into() });
                    vertex_data.push(Vertex { position: [x + width, y, z, 1.0], color: color.into() });
                    vertex_data.push(Vertex { position: [x + width, y + height, z, 1.0], color: color.into() });
                    vertex_data.push(Vertex { position: [x, y, z, 1.0], color: color.into() });
                    vertex_data.push(Vertex { position: [x + width, y + height, z, 1.0], color: color.into() });
                    vertex_data.push(Vertex { position: [x, y + height, z, 1.0], color: color.into() });
                }
                _ => {}
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
                z: self.current_depth,
                width: (box_model.padding.horizontal() + box_model.content.width) as f32,
                height: (box_model.padding.vertical() + box_model.content.height) as f32,
                color: style.background_color.clone(),
            });

            self.current_depth += 1.0;
        }

        // draw border box
        if box_model.has_border() {
            self.render_calls.push(Primitive::Rectangle {
                x: (box_model.x + box_model.margin.left) as f32,
                y: (box_model.y + box_model.margin.top) as f32,
                z: self.current_depth,
                width: (box_model.border.horizontal() + box_model.padding.horizontal() + box_model.content.width) as f32,
                height: (box_model.border.vertical() + box_model.padding.vertical() + box_model.content.height) as f32,
                color: style.border_color.clone(),
            });

            self.current_depth += 1.0;
        }

        // draw margin box
        if box_model.has_margin() {
            self.render_calls.push(Primitive::Rectangle {
                x: box_model.x as f32,
                y: box_model.y as f32,
                z: self.current_depth,
                width: box_model.width() as f32,
                height: box_model.height() as f32,
                color: COLOR_TRANSPARENT,
            });

            self.current_depth += 1.0;
        }
    }
    pub fn draw_text(&mut self, value: &String, style: &Style) {
        if value.trim().len() <= 0 {
            return;
        }

        self.render_calls.push(
            Primitive::Text {
                x: style.box_model.x as f32,
                y: style.box_model.y as f32,
                z: self.current_depth,
                value: value.clone(),
                width: style.box_model.width() as f32,
                height: style.box_model.height() as f32,
                family: style.font_family.clone(),
                font_size: style.font_size.to_pixels() as f32,
                weight: style.font_weight,
                line_height: style.line_height.to_pixels() as f32,
                color: style.text_color,
            }
        );

        self.current_depth += 1.0;
    }
}
