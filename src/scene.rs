pub struct Scene {
    grid: crate::grid::Grid,
}

pub trait Placeable {
    fn place(&self) -> Place;
}

pub type PlaceParam = f32;
pub type Place = [PlaceParam; 4];

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
