pub mod color;
pub use color::*;
mod sprite;
use sprite::*;
pub mod framework_bridge;
use framework_bridge::*;

const CONFIG_PATH: &str = "config.toml";

#[derive(serde::Deserialize)]
struct Config {
    window_size: [u16; 2],
    grid_size: [usize; 2],
}

const BACKGROUND_COLOR: Color = BLACK;

#[macroquad::main("Sandbog")]
async fn main() {
    let config = load_config(CONFIG_PATH);

    // set window size
    let [window_width, window_height] = config.window_size;
    set_window_size(window_width, window_height);

    let [grid_width, grid_height] = config.grid_size;
    let grid = Sprite {
        x: 0.,
        y: 0.,
        w: grid_width,
        h: grid_height,
        pixels: vec![Color::default(); grid_width * grid_height],
    };

    loop {
        // UPDATE

        // DRAW
        clear_background(BACKGROUND_COLOR);
        grid.draw();

        macroquad::window::next_frame().await
    }
}

fn load_config(path: &str) -> Config {
    let config_string = std::fs::read_to_string(path).unwrap();
    toml::from_str(&config_string).unwrap()
}
