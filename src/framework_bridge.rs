use crate::{color::Color, sprite::Sprite};

type WindowSizeType = u16;

pub fn set_window_size(w: WindowSizeType, h: WindowSizeType) {
    macroquad::window::request_new_screen_size(w as f32, h as f32);
}

pub fn clear_background(color: Color) {
    macroquad::window::clear_background(macroquad::color::Color::from(color));
}

impl From<Color> for macroquad::color::Color {
    fn from(color: Color) -> Self {
        let [r, g, b] = color.to_array();
        Self::from_rgba(r, g, b, 255)
    }
}

impl Sprite {
    pub fn draw(&self) {
        let texture = macroquad::texture::Texture2D::from(self);
        macroquad::texture::draw_texture(&texture, self.x, self.y, macroquad::color::WHITE);
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        for pixel in &self.pixels {
            bytes.extend_from_slice(&pixel.to_array());
            bytes.push(255);
        }
        bytes
    }
}

impl From<&Sprite> for macroquad::texture::Texture2D {
    fn from(sprite: &Sprite) -> Self {
        Self::from_rgba8(sprite.w as u16, sprite.h as u16, &sprite.to_bytes())
    }
}
