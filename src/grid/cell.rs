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
    pub fn split(self, cells_wide: u32, cells_tall: u32) -> Vec<Self> {
        let width = self.width * cells_wide;
        let height = self.height * cells_tall;
        let mut cells = vec![];
        for ix in 0..cells_wide {
            for iy in 0..cells_tall {
                let x = self.x + ix * width;
                let y = self.y + iy * height;
                let cell = Self::new(x, y, width, height, self.color);
                cells.push(cell);
            }
        }
        cells
    }
}
