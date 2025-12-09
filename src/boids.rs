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
//! // Initialise settings
//! let boid_settings = BoidSettings::new(10f, 5f, 10, 10);
//! boid_settings
//!     .set_gravity(GRAVITY)
//!     .set_min_speed(MIN_SPEED)
//!     .set_border(BorderSettings::Bounded {
//!         turn_force: TURN_FORCE,
//!         margin: MARGIN,
//!     });
//!
//! // Create population
//! let population = populate(COUNT, GROUP_COUNT, &boid_settings);
//! // Update one time step
//! update_boids(&mut population, &boid_settings, DELTA_TIME);
//! ```

use crate::grid::{Grid, ValueNode};
use crate::vector2::Vector2;
pub use settings::{BoidSettings, BorderSettings};

pub mod settings;
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
    group: u8,
}

impl Boid {
    pub fn new(position: Vector2, velocity: Vector2, group: u8) -> Boid {
        Boid {
            position,
            velocity,
            group,
        }
    }
}

#[inline]
fn get_grid_position(
    position: Vector2,
    boid_settings: &BoidSettings,
    grid: &Grid<super::Boid>,
) -> (i32, i32) {
    let width = boid_settings.width;
    let height = boid_settings.height;

    let grid_row = (position.y / height as f32 * grid.rows as f32) as i32;
    let grid_column = (position.x / width as f32 * grid.columns as f32) as i32;
    (grid_row, grid_column)
}

/// Initialises a new grid according to the defined number of cells within the
/// affecting radius of a boid and width and height in the `boid_settings`.
fn grid_init(count: usize, boid_settings: &BoidSettings) -> Grid<Boid> {
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
    Grid::new(count, grid_columns, grid_rows)
}

/// Creates a new population of `count` number boids divided equally among
/// `group_count` groups.The created population is stored in a grid with the
/// appropiate cell size for boids to have `CELLS_IN_RADIUS` number of cells
/// within any direction of the boids visual range.
///
/// ## Groups
/// Boids belonging to a certain group are only attracted to others in the same group.
pub fn populate(count: usize, group_count: u8, boid_settings: &BoidSettings) -> Grid<Boid> {
    let mut generator = fastrand::Rng::new();
    let mut grid = grid_init(count, boid_settings);

    let width = boid_settings.width;
    let height = boid_settings.height;

    // Populate grid with new randomly placed boids
    let velocity = Vector2 { x: 0f32, y: 0f32 };
    for i in 0..count {
        let position = Vector2 {
            x: generator.f32() * (width as f32),
            y: generator.f32() * (height as f32),
        };
        let (grid_row, grid_column) = get_grid_position(position, boid_settings, &grid);
        grid.add_val(
            Boid::new(position, velocity, (i % group_count as usize) as u8),
            grid_row,
            grid_column,
        );
    }
    grid
}

/// Resizes the grid by creating a new one according to the current
/// `boid_settings` and moving all boids to their correct positions within the new
/// grid.
fn resize_grid(grid: &mut Grid<Boid>, boid_settings: &BoidSettings) {
    let mut new_grid: Grid<Boid> = grid_init(grid.count, boid_settings);

    // Move boids from old to new grid
    for ValueNode {
        val: boid,
        next_index: _,
    } in grid.values.iter()
    {
        let position = boid.position;
        let (grid_row, grid_column) = get_grid_position(position, boid_settings, &new_grid);
        new_grid.add_val(*boid, grid_row, grid_column);
    }
    *grid = new_grid;
}

/// Update the location of every boid in the grid based on the given
/// `boid_settings` across a given `delta` time frame.
pub fn update_boids(grid: &mut Grid<Boid>, boid_settings: &BoidSettings, delta: f32) {
    let boid_count = grid.values.len();

    for i in 0..boid_count {
        simulation::update_boid(i, grid, boid_settings, delta);
    }
}
