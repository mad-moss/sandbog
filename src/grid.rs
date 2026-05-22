mod cell;
use cell::*;

pub struct Grid {
    place: Place,
    grid: Vec<Cell>,
}

impl Grid {
    pub fn add_cell(&mut self, cell: Cell) {
        self.grid.push(cell);
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
