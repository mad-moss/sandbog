use crate::{color::*, color_like::*, rule::*};

pub fn rand_rule() -> Rule {
    Rule::new(
        RuleComponent::new(1, 1, vec![ColorLike::Basic(Color::default())]),
        RuleComponent::new(
            1,
            1,
            vec![ColorLike::Complex(ComplexColor::new(
                ComplexColorComponent::Range([0, 255]),
                ComplexColorComponent::Range([0, 255]),
                ComplexColorComponent::Range([0, 255]),
                ComplexColorComponent::Basic(255),
            ))],
        ),
    )
    .unwrap()
}

pub fn sand_rule() -> Rule {
    let sand = ColorLike::Basic(YELLOW);
    let air = ColorLike::Basic(Color::default());
    Rule::new(
        RuleComponent::new(1, 2, vec![sand.clone(), air.clone()]),
        RuleComponent::new(1, 2, vec![air, sand]),
    )
    .unwrap()
}
