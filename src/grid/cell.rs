type Color = [u8; 3];

pub struct Cell {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color: Color,
}

impl Cell {
    fn new(x: u32, y: u32, width: u32, height: u32, color: Color) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
        }
    }
    pub fn color(&self) -> Color {
        self.color
    }
    pub fn recolor(&mut self, color: Color) {
        self.color = color;
    }
    pub fn split(&mut self, cells_wide: u32, cells_tall: u32) -> Vec<Self> {
        let width = self.width
        let children = vec![];
        for ix in 0..cells_wide {
            for iy in 0..cells_tall {

            }
        }
        children
    }
}
