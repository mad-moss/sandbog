use crate::Color;

pub type GridIndex = u16;

pub struct Grid {
    pub width: GridIndex,
    pub height: GridIndex,
    grid: Vec<Color>,
}

impl Grid {
    pub fn new(width: GridIndex, height: GridIndex, fill: Color) -> Self {
        Self {
            width,
            height,
            grid: vec![fill; width as usize * height as usize],
        }
    }
    pub fn get_cell(&self, x: GridIndex, y: GridIndex) -> Color {
        self.grid[(self.width * y + x) as usize]
    }
    pub fn set_cell(&mut self, x: GridIndex, y: GridIndex, color: Color) {
        self.grid[(self.width * y + x) as usize] = color;
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        for cell in &self.grid {
            bytes.extend_from_slice(&cell.to_array());
        }
        bytes
    }
}
