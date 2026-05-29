#[derive(Clone)]
pub struct Grid<T> {
    dimensions: [u16; 2],
    values: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(w: u16, h: u16, values: Vec<T>) -> Self {
        Self {
            dimensions: [w, h],
            values,
        }
    }
    pub fn get_value(&self, x: u16, y: u16) -> Result<&T, crate::OutOfBoundsError> {
        let i = self.index(x, y)?;
        Ok(&self.values[i])
    }
    pub fn set_value(&mut self, x: u16, y: u16, value: T) -> Result<(), crate::OutOfBoundsError> {
        let i = self.index(x, y)?;
        self.values[i] = value;
        Ok(())
    }
    fn index(&self, x: u16, y: u16) -> Result<usize, crate::OutOfBoundsError> {
        let [w, h] = self.dimensions;
        if x >= w || y >= h {
            return Err(crate::OutOfBoundsError);
        }
        Ok(w as usize * y as usize + x as usize)
    }
    pub fn dimensions(&self) -> [u16; 2] {
        self.dimensions
    }
    pub fn values(&self) -> &Vec<T> {
        &self.values
    }
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn blank(w: u16, h: u16, value: T) -> Self {
        let mut grid = Self {
            dimensions: [w, h],
            values: vec![],
        };
        grid.fill(value);
        grid
    }
    pub fn fill(&mut self, value: T) {
        let [w, h] = self.dimensions;
        self.values = vec![value; w as usize * h as usize];
    }
}
