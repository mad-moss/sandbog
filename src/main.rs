use macroquad::{miniquad::conf::Conf, window::next_frame};
mod grid;

const WINDOW_WIDTH: u16 = 800;
const WINDOW_HEIGHT: u16 = 600;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Sandbog"),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    loop {
        // UPDATE

        // DRAW

        next_frame().await
    }
}
