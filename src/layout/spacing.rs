#[derive(Debug, Default, Hash, Clone, Copy)]
pub struct Spacing {
    pub top:    usize,
    pub right:  usize,
    pub bottom: usize,
    pub left:   usize
}

impl Spacing {
    pub fn vertical(&self) -> usize {
        self.top + self.bottom
    }

    pub fn horizontal(&self) -> usize {
        self.right + self.left
    }

    pub fn set_horizontal(&mut self, val: usize) {
        self.right =    val;
        self.left =     val;
    }

    pub fn set_vertical(&mut self, val: usize) {
        self.top =      val;
        self.bottom =   val;
    }

    pub fn set(&mut self, top: usize, right: usize, bottom: usize, left: usize) {
        self.top =      top;
        self.right =    right;
        self.bottom =   bottom;
        self.left =     left;
    }
}