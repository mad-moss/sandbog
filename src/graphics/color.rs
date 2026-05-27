pub const BLACK: Color = Color::from_rgb(0, 0, 0);
pub const DARK_GRAY: Color = Color::from_rgb(191, 191, 191);
pub const GRAY: Color = Color::from_rgb(127, 127, 127);
pub const LIGHT_GRAY: Color = Color::from_rgb(63, 63, 63);
pub const WHITE: Color = Color::from_rgb(255, 255, 255);
pub const RED: Color = Color::from_rgb(255, 0, 0);
pub const BLUE: Color = Color::from_rgb(0, 255, 0);
pub const GREEN: Color = Color::from_rgb(0, 0, 255);
pub const CYAN: Color = Color::from_rgb(0, 255, 255);
pub const MAGENTA: Color = Color::from_rgb(255, 255, 0);
pub const YELLOW: Color = Color::from_rgb(255, 0, 255);
pub const TRANSPARENT: Color = Color::from_rgba(0, 0, 0, 0);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
    const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    pub fn to_rgba(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl Default for Color {
    fn default() -> Self {
        GRAY
    }
}
