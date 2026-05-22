pub use crate::scene::{Place, Placeable};

pub const DEFAULT_CELL_COLOR: Color = Color {
    r: 63,
    g: 63,
    b: 63,
};

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

pub struct Cell {
    place: Place,
    color: Color,
}

impl Cell {
    fn new(place: Place, color: Color) -> Self {
        Self { place, color }
    }
    pub fn color(&self) -> Color {
        self.color
    }
    pub fn recolor(&mut self, color: Color) {
        self.color = color;
    }
    pub fn split(self, cells_wide: u32, cells_tall: u32) -> Vec<Self> {
        let width = self.place.width / cells_wide as f32;
        let height = self.place.height / cells_tall as f32;
        let mut cells = vec![];
        for ix in 0..cells_wide {
            for iy in 0..cells_tall {
                let x = self.place.x + ix as f32 * width;
                let y = self.place.y + iy as f32 * height;
                let place = Place {
                    x,
                    y,
                    width,
                    height,
                };
                let cell = Self::new(place, self.color);
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
        let left = cell.place.x;
        let right = cell.place.x + cell.place.width;
        xs.extend_from_slice(&[left, right]);
        let top = cell.place.y;
        let bottom = cell.place.y + cell.place.height;
        ys.extend_from_slice(&[top, bottom]);
    }

    xs.sort_by(f32::total_cmp);
    ys.sort_by(f32::total_cmp);

    let x = *xs.first().unwrap();
    let y = *ys.first().unwrap();
    let width = *xs.last().unwrap();
    let height = *ys.last().unwrap();

    let place = Place {
        x,
        y,
        width,
        height,
    };
    Cell::new(place, color)
}

impl Placeable for Cell {
    fn get_place(&self) -> Place {
        self.place
    }
    fn set_place(&mut self, place: Place) {
        self.place = place;
    }
}
