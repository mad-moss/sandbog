pub mod display {
    use crate::{Sprite, Texture, color::*};

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
            let [r, g, b, a] = color.to_rgba();
            Self::from_rgba(r, g, b, a)
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
        pub fn draw_grid_lines(&self, grid_color: Color) {
            let [abs_w, abs_h] = self.transform.absolute_dimensions;
            let [w, h] = self.texture.dimensions();
            let [pixel_w, pixel_h] = [abs_w / w as f32, abs_h / h as f32];
            let [pixel_w, pixel_h] = [pixel_w.round() as u16, pixel_h.round() as u16];
            let [grid_w, grid_h] = [pixel_w * w, pixel_h * h];
            let mut pixels = vec![];
            for x in 1..=grid_w {
                let on_vertical = x % pixel_w <= 1;
                for y in 1..=grid_h {
                    let on_grid = on_vertical || y % pixel_h <= 1;
                    let pixel_color = if on_grid { grid_color } else { TRANSPARENT };
                    pixels.push(pixel_color);
                }
            }
            let grid_lines = Sprite {
                transform: self.transform,
                texture: Texture::new(grid_w, grid_h, pixels),
            };
            grid_lines.draw();
        }
    }

    impl Texture {
        pub fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = vec![];
            for pixel in self.values() {
                bytes.extend_from_slice(&pixel.to_rgba());
            }
            bytes
        }
    }

    impl From<&Texture> for macroquad::texture::Texture2D {
        fn from(texture: &Texture) -> Self {
            let [w, h] = texture.dimensions();
            Self::from_rgba8(w as u16, h as u16, &texture.to_bytes())
        }
    }
}
