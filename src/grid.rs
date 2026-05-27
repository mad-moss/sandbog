pub struct Grid<T: Clone> {
    dimensions: [u16; 2],
    values: Vec<T>,
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
    pub fn new(w: u16, h: u16, values: Vec<T>) -> Self {
        Self {
            dimensions: [w, h],
            values,
        }
    }
    pub fn fill(&mut self, value: T) {
        let [w, h] = self.dimensions;
        self.values = vec![value; w as usize * h as usize];
    }
    pub fn get_value(&self, x: u16, y: u16) -> &T {
        let i = self.index(x, y);
        &self.values[i]
    }
    pub fn set_value(&mut self, x: u16, y: u16, value: T) {
        let i = self.index(x, y);
        self.values[i] = value;
    }
    fn index(&self, x: u16, y: u16) -> usize {
        let [w, h] = self.dimensions;
        assert!(x < w && y < h, "index out of bounds");
        w as usize * y as usize + x as usize
    }
    pub fn dimensions(&self) -> [u16; 2] {
        self.dimensions
    }
    pub fn values(&self) -> &Vec<T> {
        &self.values
    }
}
