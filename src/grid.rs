use crate::Element;

pub struct Grid {
    width: u16,
    height: u16,
    grid: Vec<Element>,
}

impl Grid {
    pub fn new(width: u16, height: u16, fill: Element) -> Self {
        Self {
            width,
            height,
            grid: vec![fill; width as usize * height as usize],
        }
    }
    pub fn as_slice(&self) -> Vec<u8> {
        let mut bytes = vec![];
        for cell in &self.grid {
            bytes.extend_from_slice(&cell.as_slice());
        }
        bytes
    }
}
