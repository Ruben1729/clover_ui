#[derive(Debug, Default)]
pub struct SpacingPx {
    pub top: isize,
    pub right: isize,
    pub bottom: isize,
    pub left: isize,
}

impl SpacingPx {
    pub fn vertical(&self) -> isize {
        self.top + self.bottom
    }
    pub fn horizontal(&self) -> isize {
        self.right + self.left
    }
    pub fn set_horizontal(&mut self, new_margin: isize) {
        self.right = new_margin;
        self.left = new_margin;
    }
    pub fn set_vertical(&mut self, new_margin: isize) {
        self.top = new_margin;
        self.bottom = new_margin;
    }
}

#[derive(Debug, Default)]
pub struct BoundsPx {
    pub width: isize,
    pub height: isize
}

#[derive(Debug, Default)]
pub struct BoxModel {
    pub margin: SpacingPx,
    pub border: SpacingPx,
    pub padding: SpacingPx,
    pub content: BoundsPx,
    pub x: isize,
    pub y: isize
}

impl BoxModel {
    pub fn width(&self) -> isize {
        self.margin.horizontal() + self.border.horizontal() + self.padding.horizontal() + self.content.width
    }
    pub fn height(&self) -> isize {
        self.margin.vertical() + self.border.vertical() + self.padding.vertical() + self.content.height
    }
    pub fn has_padding(&self) -> bool {
        (self.padding.horizontal() + self.padding.vertical()) > 0
    }
    pub fn has_border(&self) -> bool {
        (self.border.horizontal() + self.border.vertical()) > 0
    }
    pub fn has_margin(&self) -> bool {
        (self.margin.horizontal() + self.margin.vertical()) > 0
    }
}
