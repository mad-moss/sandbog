use crate::Color;

type SpritePosition = f32;
type SpriteIndex = u16;

pub struct Sprite {
    pub x: SpritePosition,
    pub y: SpritePosition,
    pub w: SpriteIndex,
    pub h: SpriteIndex,
    pixels: Vec<Color>,
}

impl Sprite {
    pub fn new(
        x: SpritePosition,
        y: SpritePosition,
        w: SpriteIndex,
        h: SpriteIndex,
        fill: Color,
    ) -> Self {
        Self {
            x,
            y,
            w,
            h,
            pixels: vec![fill; w as usize * h as usize],
        }
    }
    pub fn get_pixel(&self, x: SpriteIndex, y: SpriteIndex) -> Color {
        self.pixels[(self.w * y + x) as usize]
    }
    pub fn set_pixel(&mut self, x: SpriteIndex, y: SpriteIndex, color: Color) {
        self.pixels[(self.w * y + x) as usize] = color;
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        for pixel in &self.pixels {
            bytes.extend_from_slice(&pixel.to_array());
        }
        bytes
    }
}
