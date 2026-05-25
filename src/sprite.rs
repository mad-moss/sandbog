use crate::Color;

type SpritePosition = f32;

pub struct Sprite {
    pub x: SpritePosition,
    pub y: SpritePosition,
    pub w: usize,
    pub h: usize,
    pub pixels: Vec<Color>,
}

impl Sprite {
    pub fn fill(&mut self, color: Color) {
        self.pixels = vec![color; self.w * self.h];
    }
    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.pixels[self.w * y + x]
    }
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[self.w * y + x] = color;
    }
}
