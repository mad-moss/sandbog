pub mod framework_bridge;
pub mod graphics;
pub mod sprite;
pub mod transform;
pub use crate::{framework_bridge::*, graphics::*, sprite::*, transform::*};

const CONFIG_PATH: &str = "config.toml";

#[derive(serde::Deserialize)]
struct Config {
    window_size: [u16; 2],
    grid_size: [u16; 2],
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

    let [grid_w, grid_h] = config.grid_size;
    let grid = Sprite {
        texture: Texture::blank(grid_w, grid_h, Color::default()),
        transform: Transform::default(),
    };

    loop {
        // UPDATE

        // DRAW
        clear_background(BACKGROUND_COLOR);
        grid.draw();

        macroquad::window::next_frame().await
    }
}
