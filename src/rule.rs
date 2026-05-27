use crate::{Color, Grid};

type RuleComponent = Grid<Option<ColorLike>>;

struct Rule {
    condition: RuleComponent,
    result: RuleComponent,
}

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
    fn to_rgba(&self) -> [&ComplexColorComponent; 4] {
        [&self.r, &self.g, &self.b, &self.a]
    }
}

enum ColorLike {
    Basic(Color),
    Multi(Vec<Self>),
    Complex(ComplexColor),
}

impl ColorLike {
    fn is_or_contains(&self, given_color: &Color) -> bool {
        match self {
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
                let these_components = this_color.to_rgba();
                let given_components = given_color.to_rgba();
                for i in 0..=3 {
                    let this_component = these_components[i];
                    let given_component = given_components[i];
                    if this_component.is_or_contains(&given_component) {
                        return true;
                    }
                }
                false
            }
        }
    }
}
