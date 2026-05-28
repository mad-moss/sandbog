use crate::Color;

#[derive(Clone)]
pub enum ComplexColorComponent {
    Basic(u8),
    Multi(Vec<Self>),
    Range([u8; 2]),
}

impl ComplexColorComponent {
    pub fn is_or_contains(&self, given_component: &u8) -> bool {
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
    pub fn pick_value(&self) -> u8 {
        match self {
            ComplexColorComponent::Basic(value) => *value,
            ComplexColorComponent::Multi(values) => {
                let rand_value = macroquad::rand::ChooseRandom::choose(values.as_slice()).unwrap();
                rand_value.pick_value()
            }
            ComplexColorComponent::Range(range) => {
                macroquad::rand::RandomRange::gen_range(range[0], range[1])
            }
        }
    }
}

#[derive(Clone)]
pub struct ComplexColor {
    r: ComplexColorComponent,
    g: ComplexColorComponent,
    b: ComplexColorComponent,
    a: ComplexColorComponent,
}

impl ComplexColor {
    pub fn new(
        r: ComplexColorComponent,
        g: ComplexColorComponent,
        b: ComplexColorComponent,
        a: ComplexColorComponent,
    ) -> Self {
        Self { r, g, b, a }
    }
    pub fn as_rgba(&self) -> [&ComplexColorComponent; 4] {
        [&self.r, &self.g, &self.b, &self.a]
    }
}

#[derive(Clone)]
pub enum ColorLike {
    Empty,
    Basic(Color),
    Multi(Vec<Self>),
    Complex(ComplexColor),
}

impl ColorLike {
    pub fn is_or_contains(&self, given_color: &Color) -> bool {
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
    pub fn pick_color(&self) -> Option<Color> {
        match self {
            ColorLike::Empty => None,
            ColorLike::Basic(color) => Some(*color),
            ColorLike::Multi(color_likes) => {
                let rand_color_like =
                    macroquad::rand::ChooseRandom::choose(color_likes.as_slice()).unwrap();
                rand_color_like.pick_color()
            }
            ColorLike::Complex(complex_color) => {
                let complex_components = complex_color.as_rgba();
                let mut new_components: [u8; 4] = [0; 4];
                for i in 0..4 {
                    new_components[i] = complex_components[i].pick_value();
                }
                Some(Color::from(new_components))
            }
        }
    }
}
