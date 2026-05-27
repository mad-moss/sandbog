use crate::{Color, Grid, Texture};

enum ComplexColorComponent {
    Basic(u8),
    Multi(Vec<Self>),
    Range(std::ops::RangeInclusive<u8>),
}

impl ComplexColorComponent {
    fn is_or_contains(&self, given_component: &u8) -> bool {
        match self {
            ComplexColorComponent::Basic(this_component) => this_component == given_component,
            ComplexColorComponent::Multi(these_components) => {
                for this_component in these_components {
                    if this_component.is_or_contains(given_component) {
                        return true;
                    }
                }
                false
            }
            ComplexColorComponent::Range(range) => range.contains(given_component),
        }
    }
}

struct ComplexColor {
    r: ComplexColorComponent,
    g: ComplexColorComponent,
    b: ComplexColorComponent,
    a: ComplexColorComponent,
}

impl ComplexColor {
    fn as_rgba(&self) -> [&ComplexColorComponent; 4] {
        [&self.r, &self.g, &self.b, &self.a]
    }
}

enum ColorLike {
    Empty,
    Basic(Color),
    Multi(Vec<Self>),
    Complex(ComplexColor),
}

impl ColorLike {
    fn is_or_contains(&self, given_color: &Color) -> bool {
        match self {
            ColorLike::Empty => true,
            ColorLike::Basic(this_color) => this_color == given_color,
            ColorLike::Multi(these_colors) => {
                for this_color in these_colors {
                    if this_color.is_or_contains(given_color) {
                        return true;
                    }
                }
                false
            }
            ColorLike::Complex(this_color) => {
                let these_components = this_color.as_rgba();
                let given_components = given_color.to_rgba();
                for i in 0..=3 {
                    let this_component = these_components[i];
                    let given_component = &given_components[i];
                    if this_component.is_or_contains(given_component) {
                        return true;
                    }
                }
                false
            }
        }
    }
}

type RuleComponent = Grid<ColorLike>;

struct Rule {
    condition: RuleComponent,
    result: RuleComponent,
}

impl Rule {
    fn check_condition(&self, grid: Texture, origin_x: u16, origin_y: u16) -> bool {
        let condition = &self.condition;
        let [condition_w, condition_h] = condition.dimensions();
        let [grid_w, grid_h] = grid.dimensions();
        let [condition_right, condition_bottom] = [origin_x + condition_w, origin_y + condition_h];
        if condition_right >= grid_w || condition_bottom >= grid_h {
            return false;
        }
        for y in 0..condition_h {
            let grid_y = origin_y + y;
            for x in 0..condition_w {
                let grid_x = origin_x + x;
                let current_grid_pixel = grid.get_value(grid_x, grid_y);
                let current_condition_pixel = condition.get_value(x, y);
                if !current_condition_pixel.is_or_contains(current_grid_pixel) {
                    return false;
                }
            }
        }
        true
    }
    fn apply_result(&self, grid: &mut Texture, origin_x: u16, origin_y: u16) {
        let condition = &self.condition;
        let [condition_w, condition_h] = condition.dimensions();
        let [grid_w, grid_h] = grid.dimensions();
        let [condition_right, condition_bottom] = [origin_x + condition_w, origin_y + condition_h];
        assert!(condition_right < grid_w && condition_bottom < grid_h);
    }
}
