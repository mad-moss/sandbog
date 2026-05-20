#[derive(Clone)]
pub struct Element {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
impl Default for Element {
    fn default() -> Self {
        Self {
            r: 64,
            g: 64,
            b: 64,
            a: 255,
        }
    }
}
impl Element {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
    pub fn as_slice(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}
