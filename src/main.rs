use macroquad::{miniquad::conf::Conf, window::next_frame};
mod grid;

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
    loop {
        // UPDATE

        // DRAW

        next_frame().await
    }
}
