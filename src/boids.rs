//! # Boids
//!
//! Contains the boid definitions and supplies functions to simulate their behavior.
//!
//! ## Simulating
//! The simulation is run by progressively stepping through time using `update_boids`.
//! This function updates the positions and velocities of the boids based on the basic
//! boid rules and the given settings.
//!
//! ## BoidSettings
//! To control the simulation, the `BoidSettings` struct must be created. This struct controls
//! global settings for every boid and can be adjusted between simulation steps alter the
//! behavior of the simulation.
//!
//! ### Example
//! ```no_run
//! let boid_settings = BoidSettings::new(10f, 5f, 10, 10);
//! boid_settings
//!     .set_gravity(GRAVITY)
//!     .set_min_speed(MIN_SPEED)
//!     .set_border(BorderSettings::Bounded {
//!         turn_force: TURN_FORCE,
//!         margin: MARGIN,
//!     });
//! ```

use crate::grid::{Grid, ValueNode};
use crate::vector2::Vector2;
pub use settings::{BoidSettings, BorderSettings};

/// Setting definitions used to control the simulation
pub mod settings;
/// Functions and rules in charge of simulating the boids
pub mod simulation;

/// The amount of cells that must be checked in any direction to cover the entire visible area of a boid
pub const CELLS_IN_RADIUS: i32 = 2;
/// The maximum amount of samples a boid can use to estimate its velocity
pub const MAX_SAMPLES: i32 = 300;

/// Simple representation of a boid
#[derive(Debug, Copy, Clone)]
pub struct Boid {
    pub position: Vector2,
    pub velocity: Vector2,
    /// Group index, the boid is only attracted by and aligning with other boids of the same group
    group: u32,
}

pub fn populate(count: usize, group_count: u32, boid_settings: &BoidSettings) -> Grid<Boid> {
    let mut generator = fastrand::Rng::new();
    let grid_columns = ((CELLS_IN_RADIUS as f32 * boid_settings.width as f32
        / boid_settings
            .visible_range
            .max(boid_settings.protected_range)) as usize)
        .max(1);
    let grid_rows = ((CELLS_IN_RADIUS as f32 * boid_settings.height as f32
        / boid_settings
            .visible_range
            .max(boid_settings.protected_range)) as usize)
        .max(1);
    let mut grid: Grid<Boid> = Grid::new(count, grid_columns, grid_rows);

    let width = boid_settings.width;
    let height = boid_settings.height;
    let velocity = Vector2 { x: 0f32, y: 0f32 };
    for i in 0..count {
        let position = Vector2 {
            x: generator.f32() * (width as f32),
            y: generator.f32() * (height as f32),
        };
        let grid_column = (position.x / width as f32 * grid.columns as f32) as i32;
        let grid_row = (position.y / height as f32 * grid.rows as f32) as i32;
        grid.add_val(
            Boid {
                position,
                velocity,
                group: i as u32 % group_count,
            },
            grid_column,
            grid_row,
        );
    }
    grid
}

// Could be more efficient, but its good enough.
fn resize_grid(grid: &mut Grid<super::Boid>, boid_settings: &BoidSettings) {
    let grid_columns = ((CELLS_IN_RADIUS as f32 * boid_settings.width as f32
        / boid_settings
            .visible_range
            .max(boid_settings.protected_range)) as usize)
        .max(1);
    let grid_rows = ((CELLS_IN_RADIUS as f32 * boid_settings.height as f32
        / boid_settings
            .visible_range
            .max(boid_settings.protected_range)) as usize)
        .max(1);
    let mut new_grid: Grid<super::Boid> = Grid::new(grid.count, grid_columns, grid_rows);
    let width = boid_settings.width;
    let height = boid_settings.height;
    for ValueNode {
        val: boid,
        next_index: _,
    } in grid.values.iter()
    {
        let position = boid.position;
        let grid_column = (position.x / width as f32 * new_grid.columns as f32) as i32;
        let grid_row = (position.y / height as f32 * new_grid.rows as f32) as i32;
        new_grid.add_val(*boid, grid_column, grid_row);
    }
    *grid = new_grid;
}

pub fn update_boids(grid: &mut Grid<Boid>, boid_settings: &BoidSettings, delta: f32) {
    let boid_count = grid.values.len();

    for i in 0..boid_count {
        simulation::update_boid(i, grid, boid_settings, delta);
    }
}
