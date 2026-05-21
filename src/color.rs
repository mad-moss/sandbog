#[derive(Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}
impl Default for Color {
    fn default() -> Self {
        Self {
            r: 64,
            g: 64,
            b: 64,
        }
    }
}
impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    pub fn as_bytes(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}
