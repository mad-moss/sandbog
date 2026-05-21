use macroquad::{texture::*, window::*};
pub mod color;
pub use color::*;
mod cell;
use cell::*;

const WIDTH: u16 = 800;
const HEIGHT: u16 = 600;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Sandbog"),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let grid = Cell::new(WIDTH, HEIGHT, Color::default());

    loop {
        // UPDATE

        // DRAW
        let texture = Texture2D::from_rgba8(WIDTH, HEIGHT, &grid.as_bytes());
        draw_texture(&texture, 0., 0., macroquad::color::WHITE);

        next_frame().await
    }
}
