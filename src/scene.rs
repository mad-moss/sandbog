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

pub mod color {

    pub const DEFAULT_CELL_COLOR: Color = Color {
        r: 63,
        g: 63,
        b: 63,
    };

    #[derive(Clone, Copy)]
    pub struct Color {
        r: u8,
        g: u8,
        b: u8,
    }
}
