use crate::Color;

pub struct Sprite {
    pub x: f32,
    pub y: f32,
    w: usize,
    h: usize,
    pixels: Vec<Color>,
}

impl Sprite {
    pub fn new(x: f32, y: f32, w: usize, h: usize, color: Color) -> Self {
        assert!(w * h <= usize::MAX, "sprite too big!");
        let pixels = vec![color; w * h];
        Self { x, y, w, h, pixels }
    }
    pub fn fill(&mut self, color: Color) {
        self.pixels = vec![color; self.w * self.h];
    }
    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        let i = self.index(x, y);
        self.pixels[i]
    }
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let i = self.index(x, y);
        self.pixels[i] = color;
    }
    fn index(&self, x: usize, y: usize) -> usize {
        assert!(x < self.w && y < self.h, "out of sprite bounds");
        self.w * y + x
    }
    pub fn dimensions(&self) -> [usize; 2] {
        [self.w, self.h]
    }
    pub fn pixels(&self) -> &Vec<Color> {
        &self.pixels
    }
}
