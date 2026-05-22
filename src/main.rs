use macroquad::window::{self, next_frame};
mod grid;

const CONFIG_PATH: &str = "config.toml";

#[derive(serde::Deserialize)]
struct Config {
    default_window_size: [u16; 2],
    default_grid_size: [u8; 2],
}

#[macroquad::main("Sandbog")]
async fn main() {
    let config_string = std::fs::read_to_string(CONFIG_PATH).unwrap();
    let config: Config = toml::from_str(&config_string).unwrap();
    let [window_width, window_height] = config.default_window_size;
    window::request_new_screen_size(window_width as f32, window_height as f32);

    loop {
        // UPDATE

        // DRAW

        next_frame().await
    }
}
