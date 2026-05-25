use crate::{Color, Texture, Transform};

pub struct Sprite {
    pub transform: Transform,
    pub texture: Texture,
}

impl Sprite {
    pub fn blank(x: f32, y: f32, w: u16, h: u16, color: Color) -> Self {
        let transform = Transform {
            position: [x, y],
            absolute_dimensions: [w as f32, h as f32],
        };
        let texture = Texture::blank(w, h, color);
        Self { transform, texture }
    }
}
