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

pub fn merge_cells(cells: Vec<Cell>, color: Color) -> Cell {
    let mut xs: Vec<f32> = vec![];
    let mut ys: Vec<f32> = vec![];
    for cell in cells {
        let left = cell.x;
        let right = cell.x + cell.width;
        xs.extend_from_slice(&[left, right]);
        let top = cell.y;
        let bottom = cell.y + cell.height;
        ys.extend_from_slice(&[top, bottom]);
    }

    xs.sort_by(f32::total_cmp);
    ys.sort_by(f32::total_cmp);

    let x = *xs.first().unwrap();
    let y = *ys.first().unwrap();
    let width = *xs.last().unwrap();
    let height = *ys.last().unwrap();

    Cell::new(x, y, width, height, color)
}
