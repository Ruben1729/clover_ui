use crate::style::Color;

#[derive(Debug, Default, Hash, Clone, Copy)]
pub struct Border {
    pub top_color: Color,
    pub right_color: Color,
    pub bottom_color: Color,
    pub left_color: Color,
    pub style: usize,
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
    pub left: usize,
}

impl Border {
    pub fn new(width: usize, color: Color) -> Border {
        Border {
            top_color: Color::new_u32(color.get_u32()),
            right_color: Color::new_u32(color.get_u32()),
            bottom_color: Color::new_u32(color.get_u32()),
            left_color: color,
            style: 0,
            top: width,
            right: width,
            bottom: width,
            left: width,
        }
    }
    pub fn set_width(&mut self, width: usize) {
        self.top = width;
        self.right = width;
        self.bottom = width;
        self.left = width;
    }
    pub fn color(&self) -> (u32, u32, u32, u32) {
        (
            self.top_color.get_u32(),
            self.right_color.get_u32(),
            self.bottom_color.get_u32(),
            self.left_color.get_u32(),
        )
    }
    pub fn set_color(&mut self, color: Color) {
        self.top_color.set_u32(color.get_u32());
        self.right_color.set_u32(color.get_u32());
        self.bottom_color.set_u32(color.get_u32());
        self.left_color.set_u32(color.get_u32())
    }
    pub fn vertical(&self) -> usize {
        self.top + self.bottom
    }
    pub fn horizontal(&self) -> usize {
        self.right + self.left
    }
}
