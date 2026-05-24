pub mod color;
pub use color::*;
mod sprite;
use sprite::*;
pub mod framework_bridge;
use framework_bridge::*;

const CONFIG_PATH: &str = "config.toml";

#[derive(serde::Deserialize)]
struct Config {
    default_window_size: [u16; 2],
    default_grid_size: [u16; 2],
}

const BACKGROUND_COLOR: Color = BLACK;

#[macroquad::main("Sandbog")]
async fn main() {
    let config = load_config(CONFIG_PATH);

    // set window size
    let [window_width, window_height] = config.default_window_size;
    set_window_size(window_width, window_height);

    let [grid_width, grid_height] = config.default_grid_size;
    let grid = Sprite::new(0., 0., grid_width, grid_height, Color::default());

    loop {
        // UPDATE

        // DRAW
        clear_background(BACKGROUND_COLOR);
        let texture = macroquad::texture::Texture2D::from_rgba8(grid.w, grid.h, &grid.to_bytes());
        macroquad::texture::draw_texture(&texture, 0., 0., macroquad::color::WHITE);

        macroquad::window::next_frame().await
    }
}

fn load_config(path: &str) -> Config {
    let config_string = std::fs::read_to_string(path).unwrap();
    toml::from_str(&config_string).unwrap()
}
