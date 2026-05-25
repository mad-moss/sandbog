pub mod framework_bridge;
pub mod graphics;
pub mod sprite;
pub mod transform;
pub use crate::{framework_bridge::display::*, graphics::*, sprite::*, transform::*};

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
    let [grid_x, grid_y] = [0., 0.];
    let mut grid = Sprite::blank(grid_x, grid_y, grid_w, grid_h, Color::default());

    loop {
        // UPDATE
        let [window_w, window_h] = window_size();
        grid.transform.scale_to_fit(window_w, window_h);

        // DRAW
        clear_background(BACKGROUND_COLOR);
        grid.draw();
        grid.draw_grid_lines(BACKGROUND_COLOR);

        macroquad::window::next_frame().await
    }
}
