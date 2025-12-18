use crate::boids::{Boid, BoidSettings};
use crate::grid::Grid;
use crate::menu::{Menu, MenuItem};

enum MenuID {
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
    SquaredFriction,

    MouseRange,
    MouseForce,
}

/// TODO:.
fn on_menu_change<'a>(
    changed_item: &MenuItem<'a, MenuID>,
    boid_settings: &mut BoidSettings,
    boid_data: &mut Grid<Boid>,
) {
    match changed_item {
        MenuItem::IntSlider { id, current, .. } => match id {
            _ => (),
        },
        MenuItem::FloatSlider { id, current, .. } => match id {
            MenuID::SeparationDistance => boid_settings.set_protected_range(*current, boid_data),
            MenuID::CohesionForce => boid_settings.set_visible_range(*current, boid_data),
            _ => (),
        },
        MenuItem::Toggle { id, current } => match id {
            _ => (),
        },
        MenuItem::Choice { id, current, .. } => match id {
            _ => (),
        },
    }
}

/// TODO:.
fn setup_menu<'a>() -> Menu<'a, MenuID> {
    let mut menu = Menu::new();
    menu.add_menu_item(
        MenuItem::FloatSlider {
            id: MenuID::SeparationDistance,
            current: SEPERATION_DIST,
            min: 1.0,
            max: 20.0,
            step_size: 0.1,
        },
        "Separation Distance",
    );
    menu.add_menu_item(
        MenuItem::FloatSlider {
            id: MenuID::CohesionDistance,
            current: COHESION_DIST,
            min: 0.0,
            max: 100.0,
            step_size: 0.1,
        },
        "Cohesion Distance",
    );
    menu
}
