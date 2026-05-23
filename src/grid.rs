mod cell;
use crate::scene::Drawable;
use cell::*;

pub struct Grid {
    x: PlaceParam,
    y: PlaceParam,
    w: PlaceParam,
    h: PlaceParam,
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new(place: Place) -> Self {
        let [x, y, w, h] = place;
        Self {
            x,
            y,
            w,
            h,
            cells: vec![],
        }
    }
    pub fn init(&mut self, cells_wide: u32, cells_tall: u32) {
        let cell_prime = Cell::new([0., 0., 1., 1.], DEFAULT_CELL_COLOR);
        self.cells = cell_prime.split(cells_wide, cells_tall);
    }
    pub fn add_cell(&mut self, cell: Cell) {
        self.cells.push(cell);
    }
}

impl Placeable for Grid {
    fn place(&self) -> Place {
        [self.x, self.y, self.w, self.h]
    }
}

impl Drawable for Grid {
    fn draw(&self) {
        for cell in &self.cells {
            macroquad::shapes::draw_rectangle(
                self.x + cell.x,
                self.y + cell.y,
                self.w * cell.w,
                self.h * cell.h,
                macroquad::color::Color::from(cell.color),
            );
        }
    }
}
