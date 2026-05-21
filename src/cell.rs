use crate::Color;

pub struct Cell {
    width: u16,
    height: u16,
    grid: Vec<Color>,
}

impl Cell {
    pub fn new(width: u16, height: u16, fill: Color) -> Self {
        Self {
            width,
            height,
            grid: vec![fill; width as usize * height as usize],
        }
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        for cell in &self.grid {
            bytes.extend(cell.as_bytes());
            bytes.push(255);
        }
        bytes
    }
}
