type Color = [u8; 3];

pub struct Cell {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
}

impl Cell {
    fn new(x: f32, y: f32, width: f32, height: f32, color: Color) -> Self {
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
        let width = self.width / cells_wide as f32;
        let height = self.height / cells_tall as f32;
        let mut cells = vec![];
        for ix in 0..cells_wide {
            for iy in 0..cells_tall {
                let x = self.x + ix as f32 * width;
                let y = self.y + iy as f32 * height;
                let cell = Self::new(x, y, width, height, self.color);
                cells.push(cell);
            }
        }
        cells
    }
}
