use crate::{Color, Sprite, Texture};

type WindowSizeType = u16;

pub fn window_size() -> [f32; 2] {
    [
        macroquad::window::screen_width(),
        macroquad::window::screen_height(),
    ]
}
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
        let macroquad_texture = macroquad::texture::Texture2D::from(&self.texture);
        let [x, y] = self.transform.position;
        let color_mask = macroquad::color::WHITE;
        let [absolute_w, absolute_h] = self.transform.absolute_dimensions;
        let params = macroquad::texture::DrawTextureParams {
            dest_size: Some(macroquad::math::Vec2::new(absolute_w, absolute_h)),
            ..Default::default()
        };

        macroquad::texture::draw_texture_ex(&macroquad_texture, x, y, color_mask, params);
    }
}

impl Texture {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        for pixel in self.pixels() {
            bytes.extend_from_slice(&pixel.to_array());
            bytes.push(255);
        }
        bytes
    }
}

impl From<&Texture> for macroquad::texture::Texture2D {
    fn from(sprite: &Texture) -> Self {
        let [w, h] = sprite.dimensions();
        Self::from_rgba8(w as u16, h as u16, &sprite.to_bytes())
    }
}
