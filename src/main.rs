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

fn load_config(path: &str) -> Config {
    let config_contents = std::fs::read_to_string(path).unwrap();
    toml::from_str(&config_contents).unwrap()
}

const BACKGROUND_COLOR: Color = BLACK;

#[macroquad::main("Sandbog")]
async fn main() {
    let config = load_config(CONFIG_PATH);

    // set window size
    let [window_width, window_height] = config.window_size;
    set_window_size(window_width, window_height);

    let [grid_x, grid_y] = [0., 0.];
    let [grid_width, grid_height] = config.grid_size;
    let grid = Sprite::new(grid_x, grid_y, grid_width, grid_height, Color::default());

    loop {
        // UPDATE

        // DRAW
        clear_background(BACKGROUND_COLOR);
        let [scale_x, scale_y] = [5., 5.];
        grid.draw_scaled(scale_x, scale_y);

        macroquad::window::next_frame().await
    }
}
