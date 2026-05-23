pub use crate::scene::{Place, PlaceParam, Placeable, color::*};

pub struct Cell {
    pub x: PlaceParam,
    pub y: PlaceParam,
    pub w: PlaceParam,
    pub h: PlaceParam,
    pub color: Color,
}

impl Cell {
    pub fn new(place: Place, color: Color) -> Self {
        let [x, y, w, h] = place;
        Self { x, y, w, h, color }
    }
    pub fn color(&self) -> Color {
        self.color
    }
    pub fn recolor(&mut self, color: Color) {
        self.color = color;
    }
    pub fn split(self, cells_wide: u32, cells_tall: u32) -> Vec<Self> {
        let width = self.w / cells_wide as PlaceParam;
        let height = self.h / cells_tall as PlaceParam;
        let mut cells = vec![];
        for ix in 0..cells_wide {
            for iy in 0..cells_tall {
                let x = self.x + ix as PlaceParam * width;
                let y = self.y + iy as PlaceParam * height;
                let cell = Self::new(self.place(), self.color);
                cells.push(cell);
            }
        }
        cells
    }
}

// FOR SURE doesn't work right
// it'll be very annoying to work out
// that's fine.
pub fn merge_cells(cells: Vec<Cell>, color: Color) -> Cell {
    let mut xs: Vec<PlaceParam> = vec![];
    let mut ys: Vec<PlaceParam> = vec![];
    for cell in cells {
        let left = cell.x;
        let right = cell.x + cell.w;
        xs.extend_from_slice(&[left, right]);
        let top = cell.y;
        let bottom = cell.y + cell.h;
        ys.extend_from_slice(&[top, bottom]);
    }

    xs.sort_by(PlaceParam::total_cmp);
    ys.sort_by(PlaceParam::total_cmp);

    let x = *xs.first().unwrap();
    let y = *ys.first().unwrap();
    let width = *xs.last().unwrap();
    let height = *ys.last().unwrap();

    let place = [x, y, width, height];
    Cell::new(place, color)
}

impl Placeable for Cell {
    fn place(&self) -> Place {
        [self.x, self.y, self.w, self.h]
    }
}
