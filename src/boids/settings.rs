//! Setting definitions used to control the simulation
//!
//! # Settings
//!
//! Contains the definitions and methods of the settings used by the boid simulation.
//! As the settings are large, only a small subset is set during initialisation
//! of the settings, requiring extra options to be activated through the setting's
//! factory pattern.

use crate::{grid::Grid, vector2::Vector2};

/// Describes the behavior of a boid near/on the border
#[allow(dead_code)]
pub enum BorderSettings {
    /// No special behavior
    None,
    /// Boids are forced away from all borders
    Bounded,
    /// Boids are forced away from the bottom and top border and wrap around the right and left ones.
    BoundedVertical,
    /// Boids are forced away from the right and left border and wrap around the bottom and top ones.
    BoundedHorizontal,
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

    // Basic Rules
    /// The cohesion force modifier
    pub cohesion: f32,
    /// The separating force modifier
    pub separation: f32,
    /// The aligning force modifier
    pub alignment: f32,

    /// Window width
    pub width: usize,
    /// Window height
    pub height: usize,

    /// Border
    pub border_settings: BorderSettings,
    /// The force normal to the border
    pub turn_force: f32,
    /// How far away from the edges of the screen the border begins.
    pub margin: f32,

    /// Gravity force, can be negative
    pub gravity: f32,
    /// Random noise applied to boid's movement
    pub noise_force: f32,
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
    /// Create a new [`BoidSettings`] object with the bare minimum initialised.
    pub fn new(
        protected_range: f32,
        visible_range: f32,
        cohesion: f32,
        separation: f32,
        alignment: f32,
        width: usize,
        height: usize,
    ) -> BoidSettings {
        BoidSettings {
            protected_range,
            visible_range,
            cohesion,
            separation,
            alignment,
            width,
            height,
            sqr_protected_range: protected_range * protected_range,
            sqr_visible_range: visible_range * visible_range,
            border_settings: BorderSettings::None,
            turn_force: 0.0,
            margin: 0.0,
            gravity: 0.0,
            min_speed: 0.0,
            noise_force: 0.0,
            friction_coefficient: 0.0,
            squared_friction: false,
            sqr_mouse_range: 0.0,
            mouse_force: 0.0,
            mouse_range: 0.0,
            mouse_position: Vector2::ZERO,
        }
    }

    /// Update the window size within which the the boids are visible.
    ///
    /// ## Side-Effect
    /// Creates a new grid to also fit the new window size.
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

    /// Set the protected range of this [`BoidSettings`].
    pub fn set_protected_range(
        &mut self,
        new_range: f32,
        grid: &mut Grid<super::Boid>,
    ) -> &mut Self {
        self.protected_range = new_range;
        self.sqr_protected_range = new_range;
        super::resize_grid(grid, self);
        self
    }

    /// Sets the visible range of this [`BoidSettings`].
    pub fn set_visible_range(&mut self, new_range: f32, grid: &mut Grid<super::Boid>) -> &mut Self {
        self.visible_range = new_range;
        self.sqr_visible_range = new_range * new_range;
        super::resize_grid(grid, self);
        self
    }

    /// Sets the cohesion force of this [`BoidSettings`].
    pub fn set_cohesion_force(&mut self, new_force: f32) -> &mut Self {
        self.cohesion = new_force;
        self
    }

    /// Sets the separation force of this [`BoidSettings`].
    pub fn set_separation_force(&mut self, new_force: f32) -> &mut Self {
        self.separation = new_force;
        self
    }

    /// Sets the alignment force of this [`BoidSettings`].
    pub fn set_alignment_force(&mut self, new_force: f32) -> &mut Self {
        self.alignment = new_force;
        self
    }

    /// Sets the gravity of this [`BoidSettings`].
    pub fn set_gravity(&mut self, gravity: f32) -> &mut Self {
        self.gravity = gravity;
        self
    }

    /// Sets the border of this [`BoidSettings`].
    ///
    /// # Examples
    /// ```
    /// border_settings.set_border(BorderSettings::Bounded)
    /// ```
    pub fn set_border(&mut self, border_settings: BorderSettings) -> &mut Self {
        self.border_settings = border_settings;
        self
    }

    /// Sets the turn force of this [`BoidSettings`].
    pub fn set_turn_force(&mut self, turn_force: f32) -> &mut Self {
        self.turn_force = turn_force;
        self
    }

    /// Sets the margin of this [`BoidSettings`].
    pub fn set_margin(&mut self, margin: f32) -> &mut Self {
        self.margin = margin;
        self
    }

    /// Sets the min speed of this [`BoidSettings`].
    pub fn set_min_speed(&mut self, min_speed: f32) -> &mut Self {
        self.min_speed = min_speed;
        self
    }

    /// Sets the noise of this [`BoidSettings`].
    pub fn set_noise(&mut self, force: f32) -> &mut Self {
        self.noise_force = force;
        self
    }

    /// Sets the friction of this [`BoidSettings`], including the friction
    /// coefficient and whether the friction scales linearly or squared with the
    /// speed, as defined by `squared_friction`.
    pub fn set_friction(&mut self, friction_coefficient: f32, squared_friction: bool) -> &mut Self {
        self.friction_coefficient = friction_coefficient;
        self.squared_friction = squared_friction;
        self
    }

    /// Sets the mouse force of this [`BoidSettings`].
    pub fn set_mouse_force(&mut self, mouse_force: f32, mouse_range: f32) -> &mut Self {
        self.mouse_force = mouse_force;
        self.mouse_range = mouse_range;
        self.sqr_mouse_range = mouse_range * mouse_range;
        self
    }

    /// Sets the mouse position of this [`BoidSettings`].
    pub fn set_mouse_position(&mut self, x: f32, y: f32) -> &mut Self {
        self.mouse_position = Vector2 { x, y };
        self
    }
}
