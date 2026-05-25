use crate::Color;

pub struct Texture {
    dimensions: [u16; 2],
    pixels: Vec<Color>,
}

impl Texture {
    pub fn blank(w: u16, h: u16, color: Color) -> Self {
        let mut texture = Self {
            dimensions: [w, h],
            pixels: vec![],
        };
        texture.fill(color);
        texture
    }
    pub fn new(w: u16, h: u16, pixels: Vec<Color>) -> Self {
        Self {
            dimensions: [w, h],
            pixels,
        }
    }
    pub fn fill(&mut self, color: Color) {
        let [w, h] = self.dimensions;
        self.pixels = vec![color; w as usize * h as usize];
    }
    pub fn get_pixel(&self, x: u16, y: u16) -> Color {
        let i = self.index(x, y);
        self.pixels[i]
    }
    pub fn set_pixel(&mut self, x: u16, y: u16, color: Color) {
        let i = self.index(x, y);
        self.pixels[i] = color;
    }
    fn index(&self, x: u16, y: u16) -> usize {
        let [w, h] = self.dimensions;
        assert!(x < w && y < h, "index out of bounds");
        w as usize * y as usize + x as usize
    }
    pub fn dimensions(&self) -> [u16; 2] {
        self.dimensions
    }
    pub fn pixels(&self) -> &Vec<Color> {
        &self.pixels
    }
}
