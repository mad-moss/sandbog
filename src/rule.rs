use crate::{ColorLike, Grid, Texture};

struct OutOfBoundsError;

pub type RuleComponent = Grid<ColorLike>;

impl RuleComponent {
    fn bounds_check(&self, grid: &Texture, x: u16, y: u16) -> Result<(), OutOfBoundsError> {
        let [w, h] = self.dimensions();
        let condition_bounds = [x + w, y + h];
        (condition_bounds <= grid.dimensions())
            .then_some(())
            .ok_or(OutOfBoundsError)
    }
}

pub struct Rule {
    condition: RuleComponent,
    result: RuleComponent,
}

impl Rule {
    pub fn new(condition: RuleComponent, result: RuleComponent) -> Result<Self, ()> {
        if condition.dimensions() != result.dimensions() {
            return Result::Err(());
        }
        Ok(Self { condition, result })
    }
    pub fn check_apply(&self, grid: &mut Texture, origin_x: u16, origin_y: u16) {
        if self.check_condition(grid, origin_x, origin_y) {
            self.apply_result(grid, origin_x, origin_y);
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
                let current_grid_pixel = grid.get_value(x_on_grid, y_on_grid);
                let current_condition_pixel = condition.get_value(x, y);
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
    ) -> Result<(), OutOfBoundsError> {
        let result = &self.result;
        result.bounds_check(grid, origin_x, origin_y)?;
        let [result_w, result_h] = result.dimensions();

        for y in 0..result_h {
            let y_on_grid = origin_y + y;
            for x in 0..result_w {
                let x_on_grid = origin_x + x;
                let new_pixel = result
                    .get_value(x, y)
                    .pick_color()
                    .unwrap_or_else(|| *grid.get_value(x, y));
                grid.set_value(x_on_grid, y_on_grid, new_pixel);
            }
        }
        Ok(())
    }
}
