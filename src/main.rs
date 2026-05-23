// REMOVE THIS
#![allow(dead_code, unused_imports, unused_variables)]
// REMOVE THAT

pub mod grid;
pub mod scene;
use crate::{grid::Grid, scene::Scene};
use macroquad::window;

const CONFIG_PATH: &str = "config.toml";

#[derive(serde::Deserialize)]
struct Config {
    default_window_size: [u16; 2],
    default_grid_size: [u32; 2],
}

#[macroquad::main("Sandbog")]
async fn main() {
    let config = load_config(CONFIG_PATH);

    // set window size
    let [window_width, window_height] = config.default_window_size;
    window::request_new_screen_size(window_width as f32, window_height as f32);

    loop {
        // UPDATE

        // DRAW

        window::next_frame().await
    }
}

fn load_config(path: &str) -> Config {
    let config_string = std::fs::read_to_string(path).unwrap();
    toml::from_str(&config_string).unwrap()
}

fn fit_to_window(scene: Scene) {
    let window_width = window::screen_width();
    let window_height = window::screen_height();
}
