pub mod color;
pub use color::*;
pub mod color_like;
pub use color_like::*;

pub type Texture = crate::Grid<Color>;
