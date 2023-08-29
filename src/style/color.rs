#[derive(Debug, Default, Hash, Clone, Copy)]
pub struct Color {
    value: u32,
}

impl Color {
    pub fn new(a: u8, r: u8, g: u8, b: u8) -> Self {
        let value = ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        Self { value }
    }
    pub fn new_u32(val: u32) -> Self {
        Color { value: val }
    }
    pub fn get_u32(&self) -> u32 {
        self.value
    }
    pub fn set_u32(&mut self, val: u32) {
        self.value = val
    }
    pub fn set_alpha(&mut self, a: u8) {
        self.value = (self.value & 0x00FFFFFF) | ((a as u32) << 24);
    }

    pub fn set_red(&mut self, r: u8) {
        self.value = (self.value & 0xFF00FFFF) | ((r as u32) << 16);
    }

    pub fn set_green(&mut self, g: u8) {
        self.value = (self.value & 0xFFFF00FF) | ((g as u32) << 8);
    }

    pub fn set_blue(&mut self, b: u8) {
        self.value = (self.value & 0xFFFFFF00) | (b as u32);
    }

    pub fn alpha(&self) -> u8 {
        ((self.value >> 24) & 0xFF) as u8
    }

    pub fn red(&self) -> u8 {
        ((self.value >> 16) & 0xFF) as u8
    }

    pub fn green(&self) -> u8 {
        ((self.value >> 8) & 0xFF) as u8
    }

    pub fn blue(&self) -> u8 {
        (self.value & 0xFF) as u8
    }
}
