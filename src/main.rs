use macroquad::{
    miniquad::conf::Conf,
    texture::{Texture2D, draw_texture},
    window::{clear_background, next_frame},
};
pub mod color;
pub use color::*;
mod grid;
use grid::*;

const WIDTH: u16 = 800;
const HEIGHT: u16 = 600;
const BACKGROUND_COLOR: Color = BLACK;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Sandbog"),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let grid = Grid::new(WIDTH, HEIGHT, Color::default());

    loop {
        // UPDATE

        // DRAW
        clear_background(macroquad::color::Color::from(BACKGROUND_COLOR));
        let texture = Texture2D::from_rgba8(WIDTH, HEIGHT, &grid.to_bytes());
        draw_texture(&texture, 0., 0., macroquad::color::WHITE);

        next_frame().await
    }
}
