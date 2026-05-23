pub struct Scene {
    grid: crate::grid::Grid,
}

#[derive(Clone, Copy)]
pub struct Place {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub trait Placeable {
    fn get_place(&self) -> Place;
    fn set_place(&mut self, place: Place);
}

pub trait Drawable {
    fn draw(&self);
}

pub mod color {

    pub const DEFAULT_CELL_COLOR: Color = Color {
        r: 63,
        g: 63,
        b: 63,
    };

    #[derive(Clone, Copy)]
    pub struct Color {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }

    impl From<Color> for macroquad::color::Color {
        fn from(color: Color) -> Self {
            macroquad::color::Color::from_rgba(color.r, color.g, color.b, 255)
        }
    }
}
