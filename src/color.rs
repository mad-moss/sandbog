pub const BLACK: Color = Color::new(0, 0, 0);
pub const DARK_GRAY: Color = Color::new(191, 191, 191);
pub const GRAY: Color = Color::new(127, 127, 127);
pub const LIGHT_GRAY: Color = Color::new(63, 63, 63);
pub const WHITE: Color = Color::new(255, 255, 255);
pub const RED: Color = Color::new(255, 0, 0);
pub const BLUE: Color = Color::new(0, 255, 0);
pub const GREEN: Color = Color::new(0, 0, 255);
pub const CYAN: Color = Color::new(0, 255, 255);
pub const MAGENTA: Color = Color::new(255, 255, 0);
pub const YELLOW: Color = Color::new(255, 0, 255);

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    pub fn to_array(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

impl Default for Color {
    fn default() -> Self {
        GRAY
    }
}
