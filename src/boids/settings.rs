use crate::{grid::Grid, vector2::Vector2};

/// Describes the behavior of a boid near/on the border
#[allow(dead_code)]
pub enum BorderSettings {
    /// No special behavior
    None,
    /// Boids are forced away from all borders
    Bounded { turn_force: f32, margin: f32 },
    /// Boids are forced away from the bottom and top border and wrap around the right and left ones.
    BoundedVertical { turn_force: f32, margin: f32 },
    /// Boids are forced away from the right and left border and wrap around the bottom and top ones.
    BoundedHorizontal { turn_force: f32, margin: f32 },
    /// Boids wrap around all borders
    Wrapping,
}

/// Contains the different settings relevant to the simulation of the boids.
/// These include both required settings such as visibility range, and border settings
/// , but also optional ones that can be changed using the implemented factory methods.
pub struct BoidSettings {
    /// Range within boids are repelled
    pub protected_range: f32,
    /// Range within boids are attracted and aligned
    pub visible_range: f32,

    /// Window width
    pub width: usize,
    /// Window height
    pub height: usize,

    /// Border
    pub border_settings: BorderSettings,
    /// Gravity force, can be negative
    pub gravity: f32,
    /// Random noise applied to boid's movement
    pub noise_force: Option<f32>,
    /// Min Speed
    pub min_speed: f32,
    /// Friction
    pub friction_coefficient: f32,
    /// Whether the friction scales polynomialy or linearly
    pub squared_friction: bool,

    // Mouse
    /// How much a boid is attracted to the mouse position, or repelled if negative
    pub mouse_force: f32,
    /// From how far the mouse has an effect
    pub mouse_range: f32,
    /// The current mouse position.
    pub mouse_position: Vector2,

    // Pre-calculations
    pub sqr_protected_range: f32,
    pub sqr_visible_range: f32,
    pub sqr_mouse_range: f32,
}

impl BoidSettings {
    /// Create a new `BoidSettings` object with the bare minimum initialised.
    pub fn new(
        protected_range: f32,
        visible_range: f32,
        width: usize,
        height: usize,
    ) -> BoidSettings {
        BoidSettings {
            protected_range,
            visible_range,
            width,
            height,
            sqr_protected_range: protected_range * protected_range,
            sqr_visible_range: visible_range * visible_range,
            border_settings: BorderSettings::None,
            gravity: 0.0,
            min_speed: 0.0,
            noise_force: None,
            friction_coefficient: 0.0,
            squared_friction: false,
            sqr_mouse_range: 0.0,
            mouse_force: 0.0,
            mouse_range: 0.0,
            mouse_position: Vector2::ZERO,
        }
    }

    /// Update the window size in which the
    pub fn update_window(
        &mut self,
        width: usize,
        height: usize,
        grid: &mut Grid<super::Boid>,
    ) -> &mut Self {
        self.width = width;
        self.height = height;
        super::resize_grid(grid, self);
        self
    }

    pub fn set_gravity(&mut self, gravity: f32) -> &mut Self {
        self.gravity = gravity;
        self
    }

    pub fn set_border(&mut self, border_settings: BorderSettings) -> &mut Self {
        self.border_settings = border_settings;
        self
    }

    pub fn set_min_speed(&mut self, min_speed: f32) -> &mut Self {
        self.min_speed = min_speed;
        self
    }

    pub fn set_noise(&mut self, force: f32) -> &mut Self {
        self.noise_force = Some(force);
        self
    }

    pub fn set_friction(&mut self, friction_coefficient: f32, squared_friction: bool) -> &mut Self {
        self.friction_coefficient = friction_coefficient;
        self.squared_friction = squared_friction;
        self
    }

    pub fn set_mouse_force(&mut self, mouse_force: f32, mouse_range: f32) -> &mut Self {
        self.mouse_force = mouse_force;
        self.mouse_range = mouse_range;
        self.sqr_mouse_range = mouse_range * mouse_range;
        self
    }

    pub fn set_mouse_position(&mut self, x: f32, y: f32) -> &mut Self {
        self.mouse_position = Vector2 { x, y };
        self
    }
}
