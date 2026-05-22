mod cell;
use cell::*;

pub struct Grid {
    place: Place,
    cells: Vec<Cell>,
}

impl Grid {
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
