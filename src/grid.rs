mod cell;
use cell::*;

pub struct Grid {
    place: Place,
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new(place: Place) -> Self {
        Self {
            place,
            cells: vec![],
        }
    }
    pub fn init(&mut self, cells_wide: u32, cells_tall: u32) {
        let cell_prime = Cell::new(
            Place {
                x: 0.,
                y: 0.,
                width: self.place.width,
                height: self.place.height,
            },
            DEFAULT_CELL_COLOR,
        );
        self.cells = cell_prime.split(cells_wide, cells_tall);
    }
    pub fn add_cell(&mut self, cell: Cell) {
        self.cells.push(cell);
    }
}

impl Placeable for Grid {
    fn get_place(&self) -> Place {
        self.place
    }
    fn set_place(&mut self, place: Place) {
        self.place = place;
    }
}
