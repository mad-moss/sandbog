mod cell;
use cell::*;

pub struct Grid {
    grid: Vec<Cell>,
}

impl Grid {
    fn add_cell(&mut self, cell: Cell) {
        self.grid.push(cell);
    }
}
