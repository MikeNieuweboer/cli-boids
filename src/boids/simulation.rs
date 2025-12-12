//! Functions and rules to be applied to a boid's speed and position.
//!
//! # Simulation
//!
//! Contains the functions used to manipulate the boid's position and
//! velocity according to its rules, including air resistance,
//! boundary conditions and attraction to other boids. To apply
//! these rules to a boid, use [`update_boid`] with the index of
//! the boid to be adjusted.

use super::{
    CELLS_IN_RADIUS, MAX_SAMPLES, get_grid_position, settings::BoidSettings,
    settings::BorderSettings,
};
use crate::{grid::Grid, vector2::Vector2};

/// Calculate the air resistance encountered by the boid based on the `velocity`
/// vector and the air resistance parameters in the `boid_settings`. The
/// calculated air resistance in both x and y is then returned as a `Vector2`.
fn drag(velocity: Vector2, boid_settings: &BoidSettings) -> Vector2 {
    let k = boid_settings.friction_coefficient;

    // Square scaling (more physically accurate)
    if boid_settings.squared_friction {
        let x = velocity.x.signum() * velocity.x * velocity.x * k;
        let y = velocity.y.signum() * velocity.y * velocity.y * k;
        Vector2 { x, y }
    } else {
        // Linear scaling
        velocity * k
    }
}

/// Create a random displacement vector based on the noise force in the
/// `boid_settings` and the time time `delta`.
///
/// ## Delta
/// The force scales with the inverse of $\sqrt{\text{delta}}. This diffuse
/// scaling tries to keep the random behavior to stay relevant independent of the
/// current time delta.$
fn rand_diffuse(boid_settings: &BoidSettings, delta: f32) -> Vector2 {
    if delta > 0.0 && boid_settings.noise_force > 0.0 {
        let diffuse = f32::sqrt(delta);
        let force = boid_settings.noise_force;
        Vector2 {
            x: force * (fastrand::f32() - 0.5) / diffuse,
            y: force * (fastrand::f32() - 0.5) / diffuse,
        }
    } else {
        Vector2::ZERO
    }
}

/// Gives the force exerted by the mouse. This force depends both on the mouse
/// options in `boid_settings` and the normalised distance between the mouse and
/// the given `position`.
///
/// # Notes
/// To improve the feeling of the mouse force and create a sort of rock in a stream
/// effect, repelling mouse forces scale with the square of the normalised distance.
fn mouse_force(position: Vector2, boid_settings: &BoidSettings) -> Vector2 {
    if boid_settings.mouse_force == 0.0 {
        return Vector2::ZERO;
    }
    let mut diff = boid_settings.mouse_position - position;
    let sqr_diff = diff.sqr_magnitude();
    if sqr_diff < boid_settings.sqr_mouse_range {
        // Squared reppel force
        if boid_settings.mouse_force < 0.0 {
            let norm_diff = f32::sqrt(sqr_diff);
            diff *= (1.0 - sqr_diff / boid_settings.sqr_mouse_range) / norm_diff
                * boid_settings.mouse_force;
        } else if boid_settings.mouse_force > 0.0 {
            diff *= (1.0 / boid_settings.mouse_range) * boid_settings.mouse_force;
        }
        diff
    } else {
        Vector2::ZERO
    }
}

/// Gives the force exerted by the border of the screen given the `position`.
/// This force equals the border's force in `boid_settings` normal to the
/// border, along with a small force in the direction of `velocity` parallel to
/// the border, generally preventing other rules from cancelling out the border
/// force.
fn border_force(position: Vector2, velocity: Vector2, boid_settings: &BoidSettings) -> Vector2 {
    let mut accel = Vector2::ZERO;
    if let BorderSettings::Bounded { turn_force, margin }
    | BorderSettings::BoundedHorizontal { turn_force, margin } = boid_settings.border_settings
    {
        accel = Vector2::ZERO;
        if position.x < margin {
            accel.x += turn_force;
            accel.y += velocity.y.signum() * turn_force * 0.01;
        } else if position.x > (boid_settings.width as f32 - margin) {
            accel.x -= turn_force;
            accel.y += velocity.y.signum() * turn_force * 0.01;
        }
    }
    if let BorderSettings::Bounded { turn_force, margin }
    | BorderSettings::BoundedVertical { turn_force, margin } = boid_settings.border_settings
    {
        if position.y < margin {
            accel.y += turn_force;
            accel.x += velocity.x.signum() * turn_force * 0.01;
        } else if position.y > (boid_settings.height as f32 - margin) {
            accel.y -= turn_force;
            accel.x += velocity.x.signum() * turn_force * 0.01;
        }
    }
    accel
}

/// Wraps around the `position` given the border conditions in the `boid_settings`.
fn wrapping(position: &mut Vector2, boid_settings: &BoidSettings) {
    // Wrap horizontally
    if let BorderSettings::Wrapping
    | BorderSettings::BoundedVertical {
        turn_force: _,
        margin: _,
    } = boid_settings.border_settings
    {
        position.x = position.x.rem_euclid(boid_settings.width as f32);
    }

    // Wrap vertically
    if let BorderSettings::Wrapping
    | BorderSettings::BoundedHorizontal {
        turn_force: _,
        margin: _,
    } = boid_settings.border_settings
    {
        position.y = position.y.rem_euclid(boid_settings.height as f32);
    }
}

/// Returns the result of applying the three basic boid rules on the boid with the
/// given `index` in the `grid`.
/// These three rules are that each boid is:
/// - Repelled from others that are too close.
/// - Attracted to the average of the boids within their visible range.
/// - Matching their velocity with other within their visible range.
///
/// , where the repelling and attracting ranges are given in the `boid_settings`.
///
/// # Return
/// The function returns the [`Vector2`] with the rules induced force, along with
/// the index of the boid before the given boid in the grid.
fn boid_rules(
    index: usize,
    grid: &Grid<super::Boid>,
    boid_settings: &BoidSettings,
    prev_index: &mut i32,
) -> Vector2 {
    // The total amount of cells that need to be scanned either horizontally
    // or vertically.
    const LOCAL_GRID_WIDTH: usize = CELLS_IN_RADIUS as usize * 2 + 1;

    // The total amount of cells that need to be scanned
    const LOCAL_GRID_SIZE: usize = LOCAL_GRID_WIDTH * LOCAL_GRID_WIDTH;

    let boid = &grid.values[index].val;
    let position = boid.position;
    let group = boid.group;
    let (grid_row, grid_column) = get_grid_position(position, boid_settings, grid);
    let left_border = grid_column - CELLS_IN_RADIUS;
    let top_border = grid_row - CELLS_IN_RADIUS;

    // Cumulative boid count for density proportional sampling.
    let mut bins = [0.0; LOCAL_GRID_SIZE];
    // Starting indices of surrounding cells
    let mut indices = [0; LOCAL_GRID_SIZE];

    // Collect the index of the first boid in each cell in range
    for r_offset in 0..LOCAL_GRID_WIDTH as i32 {
        let other_row = top_border + r_offset;
        for c_offset in 0..LOCAL_GRID_WIDTH as i32 {
            let other_column = left_border + c_offset;
            let i = (c_offset + r_offset * LOCAL_GRID_WIDTH as i32) as usize;
            indices[i] = grid.index_from_pos(other_row, other_column);
            bins[i] = if let Some(grid_node) = grid.get_grid_node(other_row, other_column) {
                grid_node.count as f32
            } else {
                0.0
            } + if i == 0 { 0.0 } else { bins[i - 1] };
        }
    }

    let mut avg = Vector2::ZERO;
    let mut align = Vector2::ZERO;
    let mut vis_count: u16 = 0;
    let mut sep = Vector2::ZERO;
    let mut prot_count: u16 = 0;
    let mut prev_found = false;

    let increment = (bins[LOCAL_GRID_SIZE - 1] / MAX_SAMPLES as f32).max(1.0);
    let mut acc = 0.0;

    // Apply rules on surrounding cells
    for current_bin in 0..LOCAL_GRID_SIZE {
        let cell_index = indices[current_bin];
        let mut local_prev_index = Grid::<super::Boid>::EMPTY;

        // Iterate over a subset of the boids in the cell
        for boid_index in grid.iter_from_index(cell_index) {
            if acc >= bins[current_bin] && (current_bin != LOCAL_GRID_SIZE / 2 || prev_found) {
                break;
            }

            if boid_index == index {
                prev_found = true;
                acc += increment;
                *prev_index = local_prev_index;
                continue;
            }

            local_prev_index = boid_index as i32;

            if acc >= bins[current_bin] {
                // If this is reached, the only thing left is to search for the prev_index.
                continue;
            }

            let other_boid = grid.get_val(boid_index).unwrap();
            let other_position = other_boid.position;
            let diff = other_position - position;
            let distance = diff.sqr_magnitude();
            if distance < boid_settings.sqr_protected_range {
                sep -= diff;
                prot_count += 1;
            } else if distance < boid_settings.sqr_visible_range && other_boid.group == group {
                avg += diff;
                align += other_boid.velocity;
                vis_count += 1;
            }
            acc += increment;
        }
    }

    if prot_count > 0 {
        sep /= prot_count as f32;
    }

    if vis_count > 0 {
        avg /= vis_count as f32;
        align /= vis_count as f32;
    }

    avg * boid_settings.cohesion + align * boid_settings.alignment + sep * boid_settings.separation
}

/// Updates the position of a boid given by `index` in the `grid`.
/// This is done by applying all rules according to `boid_settings`, to
/// change the current velocity and position of the boid. The scale of
/// change in velocity and position are both dependent on the time `delta`.
pub fn update_boid(
    index: usize,
    grid: &mut Grid<super::Boid>,
    boid_settings: &BoidSettings,
    delta: f32,
) {
    // Basic boid forces
    let boid = &grid.values[index].val;
    let position = boid.position;
    let velocity = boid.velocity;
    let mut prev_index: i32 = Grid::<super::Boid>::EMPTY;

    let mut accel = boid_rules(index, grid, boid_settings, &mut prev_index);

    // Gravity
    accel.y += boid_settings.gravity;

    // Noise
    accel += rand_diffuse(boid_settings, delta);

    // Air Resistance
    accel -= drag(velocity, boid_settings);

    // Mouse force
    accel += mouse_force(position, boid_settings);

    // Force on screen
    accel += border_force(position, velocity, boid_settings);

    let boid = &mut grid.values[index].val;
    // Update velocity based on differentials.
    let mut velocity = boid.velocity;
    velocity += accel * delta;

    // Clipping.
    let speed = velocity.magnitude();
    if speed < boid_settings.min_speed && speed != 0.0 {
        let ratio = boid_settings.min_speed / speed;
        velocity *= ratio;
    }

    // Update position based on velocity.
    let mut new_position = boid.position;
    new_position += velocity * delta;
    wrapping(&mut new_position, boid_settings);
    boid.velocity = velocity;
    boid.position = new_position;

    // Update grid's linked list
    let (grid_row, grid_column) = get_grid_position(position, boid_settings, grid);
    let (new_grid_row, new_grid_column) = get_grid_position(new_position, boid_settings, grid);

    grid.unlink_val(index, prev_index, grid_row, grid_column);

    grid.link_val(index, new_grid_row, new_grid_column);
}
