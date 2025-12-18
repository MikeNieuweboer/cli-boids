use crate::boids::{Boid, BoidSettings};
use crate::grid::Grid;
use crate::menu::{Menu, MenuItem};
use crate::{
    ALIGNMENT_FORCE, COHESION_DIST, COHESION_FORCE, FRICTION_COEFFICIENT, GRAVITY, MARGIN,
    MIN_SPEED, NOISE_FORCE, SEPARATION_FORCE, SEPERATION_DIST, SQUARED_FRICTION, TURN_FORCE,
};

pub enum MenuID {
    SeparationDistance,
    CohesionDistance,

    SeperationForce,
    CohesionForce,
    AlignmentForce,

    MinSpeed,

    TurnForce,
    Margin,

    Gravity,
    NoiseForce,
    FrictionCoefficient,
}

/// TODO:.
pub fn on_menu_change<'a>(
    changed_item: &MenuItem<'a, MenuID>,
    boid_settings: &mut BoidSettings,
    boid_data: &mut Grid<Boid>,
) {
    if let MenuItem::FloatSlider { id, current, .. } = changed_item {
        match id {
            MenuID::SeparationDistance => {
                boid_settings.set_protected_range(*current, boid_data);
            }
            MenuID::CohesionDistance => {
                boid_settings.set_visible_range(*current, boid_data);
            }
            MenuID::CohesionForce => {
                boid_settings.set_alignment_force(*current);
            }
            MenuID::SeperationForce => {
                boid_settings.set_separation_force(*current);
            }
            MenuID::AlignmentForce => {
                boid_settings.set_alignment_force(*current);
            }
            MenuID::MinSpeed => {
                boid_settings.set_min_speed(*current);
            }
            MenuID::TurnForce => {
                boid_settings.set_turn_force(*current);
            }
            MenuID::Margin => {
                boid_settings.set_margin(*current);
            }
            MenuID::Gravity => {
                boid_settings.set_gravity(*current);
            }
            MenuID::NoiseForce => {
                boid_settings.set_noise(*current);
            }
            MenuID::FrictionCoefficient => {
                boid_settings.set_friction(*current, SQUARED_FRICTION);
            }
        }
    }
}

/// TODO:.
pub fn setup_menu<'a>() -> Menu<'a, MenuID> {
    let mut menu = Menu::new();
    menu.add_menu_item(
        MenuItem::FloatSlider {
            id: MenuID::SeparationDistance,
            current: SEPERATION_DIST,
            min: 0.0,
            max: 100.0,
            step_size: 0.1,
        },
        "Separation Distance",
    )
    .add_menu_item(
        MenuItem::FloatSlider {
            id: MenuID::CohesionDistance,
            current: COHESION_DIST,
            min: 1.0,
            max: 100.0,
            step_size: 0.1,
        },
        "Cohesion Distance",
    )
    .add_menu_item(
        MenuItem::FloatSlider {
            id: MenuID::CohesionForce,
            current: COHESION_FORCE,
            min: 0.0,
            max: 10.0,
            step_size: 0.01,
        },
        "Cohesion Force",
    )
    .add_menu_item(
        MenuItem::FloatSlider {
            id: MenuID::SeperationForce,
            current: SEPARATION_FORCE,
            min: 0.0,
            max: 10.0,
            step_size: 0.01,
        },
        "Separation Force",
    )
    .add_menu_item(
        MenuItem::FloatSlider {
            id: MenuID::AlignmentForce,
            current: ALIGNMENT_FORCE,
            min: 0.0,
            max: 10.0,
            step_size: 0.01,
        },
        "Alignment Force",
    )
    .add_menu_item(
        MenuItem::FloatSlider {
            id: MenuID::MinSpeed,
            current: MIN_SPEED,
            min: 0.0,
            max: 10.0,
            step_size: 0.1,
        },
        "Min Speed",
    )
    .add_menu_item(
        MenuItem::FloatSlider {
            id: MenuID::TurnForce,
            current: TURN_FORCE,
            min: 0.0,
            max: 10.0,
            step_size: 0.1,
        },
        "Turning force",
    )
    .add_menu_item(
        MenuItem::FloatSlider {
            id: MenuID::Margin,
            current: MARGIN,
            min: -100.0,
            max: 100.0,
            step_size: 1.0,
        },
        "Margin",
    )
    .add_menu_item(
        MenuItem::FloatSlider {
            id: MenuID::Gravity,
            current: GRAVITY,
            min: -5.0,
            max: 5.0,
            step_size: 0.01,
        },
        "Gravity",
    )
    .add_menu_item(
        MenuItem::FloatSlider {
            id: MenuID::NoiseForce,
            current: NOISE_FORCE,
            min: 0.0,
            max: 1.0,
            step_size: 0.01,
        },
        "Noise force",
    )
    .add_menu_item(
        MenuItem::FloatSlider {
            id: MenuID::FrictionCoefficient,
            current: FRICTION_COEFFICIENT,
            min: 0.0,
            max: 1.0,
            step_size: 0.01,
        },
        "Friction coefficient",
    );
    menu
}
