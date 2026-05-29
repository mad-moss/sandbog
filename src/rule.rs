use crate::{ColorLike, Grid, Texture};

pub type RuleComponent = Grid<ColorLike>;

impl RuleComponent {
    fn bounds_check(&self, grid: &Texture, x: u16, y: u16) -> Result<(), crate::OutOfBoundsError> {
        let [w, h] = self.dimensions();
        let [condition_right, condition_bottom] = [x + w, y + h];
        let [grid_w, grid_h] = grid.dimensions();
        (condition_right <= grid_w && condition_bottom <= grid_h)
            .then_some(())
            .ok_or(crate::OutOfBoundsError)
    }
}

pub struct Rule {
    condition: RuleComponent,
    result: RuleComponent,
}

impl Rule {
    pub fn new(condition: RuleComponent, result: RuleComponent) -> Result<Self, String> {
        if condition.dimensions() != result.dimensions() {
            return Result::Err(String::from(
                "rule components must have matching dimensions",
            ));
        }
        Ok(Self { condition, result })
    }
    pub fn check_apply(&self, grid: &mut Texture, origin_x: u16, origin_y: u16) {
        if self.check_condition(grid, origin_x, origin_y) {
            self.apply_result(grid, origin_x, origin_y).unwrap();
        }
    }
    fn check_condition(&self, grid: &Texture, origin_x: u16, origin_y: u16) -> bool {
        let condition = &self.condition;
        if condition.bounds_check(grid, origin_x, origin_y).is_err() {
            return false;
        }
        let [condition_w, condition_h] = condition.dimensions();
        for y in 0..condition_h {
            let y_on_grid = origin_y + y;
            for x in 0..condition_w {
                let x_on_grid = origin_x + x;
                let current_grid_pixel = grid
                    .get_value(x_on_grid, y_on_grid)
                    .expect("missing/broken bounds check?");
                let current_condition_pixel = condition
                    .get_value(x, y)
                    .expect("missing/broken bounds check?");
                if !current_condition_pixel.is_or_contains(current_grid_pixel) {
                    return false;
                }
            }
        }
        true
    }
    fn apply_result(
        &self,
        grid: &mut Texture,
        origin_x: u16,
        origin_y: u16,
    ) -> Result<(), crate::OutOfBoundsError> {
        let result = &self.result;
        result.bounds_check(grid, origin_x, origin_y)?;
        let [result_w, result_h] = result.dimensions();

        for y in 0..result_h {
            let y_on_grid = origin_y + y;
            for x in 0..result_w {
                let x_on_grid = origin_x + x;
                let new_pixel = result
                    .get_value(x, y)
                    .expect("missing/broken bounds check?")
                    .pick_color()
                    .unwrap_or_else(|| *grid.get_value(x, y).unwrap());
                grid.set_value(x_on_grid, y_on_grid, new_pixel)
                    .expect("missing/broken bounds check?");
            }
        }
        Ok(())
    }
}
