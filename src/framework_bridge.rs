use crate::color::Color;

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
