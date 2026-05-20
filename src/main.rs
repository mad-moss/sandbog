use macroquad::prelude::*;
pub mod element;
pub use element::*;
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
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let grid = Grid::new(WIDTH, HEIGHT, Element::default());

    loop {
        // UPDATE

        // DRAW
        clear_background(BACKGROUND_COLOR);
        let texture = Texture2D::from_rgba8(WIDTH, HEIGHT, &grid.as_slice());
        draw_texture(&texture, 0., 0., WHITE);

        next_frame().await
    }
}
