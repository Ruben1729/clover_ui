use crate::layout::{Color, Display, Spacing, Border};

#[derive(Default)]
pub struct ConditionalLayout {
    pub padding:            Option<Spacing>,
    pub margin:             Option<Spacing>,
    pub border:             Option<Border>,
    pub height:             Option<usize>,
    pub width:              Option<usize>,
    pub y:                  Option<usize>,
    pub x:                  Option<usize>,

    pub display:            Option<Display>,

    pub background_color:   Option<Color>,
    pub color:              Option<Color>
}

#[derive(Default, Hash, Clone, Copy)]
pub struct Layout {
    pub padding:            Spacing,
    pub margin:             Spacing,
    pub border:             Border,
    pub height:             usize,
    pub width:              usize,
    pub y:                  usize,
    pub x:                  usize,

    pub display:            Display,

    pub background_color:   Color,
    pub color:              Color
}

impl Layout {
    pub fn content_x(&self) -> usize {
        self.x + self.margin.left + self.border.left + self.padding.left
    }

    pub fn content_y(&self) -> usize {
        self.y + self.margin.top + self.border.top + self.padding.top
    }

    pub fn width(&self) -> usize {
        self.margin.horizontal() +
            self.padding.horizontal() +
            self.border.horizontal() +
            self.width
    }

    pub fn height(&self) -> usize {
        self.margin.vertical() +
            self.padding.vertical() +
            self.border.vertical() +
            self.height
    }

    pub fn color_at_px(&self, dx: usize, dy: usize) -> u32 {
        if dx < self.margin.right ||
            dy < self.margin.top ||
            dx > self.padding.horizontal()  + self.border.horizontal()  + self.margin.right + self.width ||
            dy > self.padding.vertical()    + self.border.vertical()    + self.margin.top   + self.height {
            Color::default().get_u32()
        } else if dx < self.border.right + self.margin.right ||
            dy < self.border.top + self.margin.top ||
            dx > self.padding.horizontal() + self.border.right + self.margin.right + self.width ||
            dy > self.padding.vertical() + self.border.top + self.margin.top + self.height {
            self.border.color().0
        } else {
            self.background_color.get_u32()
        }
    }
}
