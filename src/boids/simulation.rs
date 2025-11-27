use super::settings::BoidSettings;
use crate::{
    boids::{CELLS_IN_RADIUS, MAX_SAMPLES, settings::BorderSettings},
    grid::Grid,
    vector2::Vector2,
};

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
        Vector2 {
            x: velocity.x * k,
            y: velocity.y * k,
        }
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
    if delta > 0.0
        && let Some(force) = boid_settings.noise_force
    {
        let diffuse = f32::sqrt(delta);
        Vector2 {
            x: force * (fastrand::f32() - 0.5) / diffuse,
            y: force * (fastrand::f32() - 0.5) / diffuse,
        }
    } else {
        Vector2::ZERO
    }
}

/// .
fn mouse_force(position: Vector2, boid_settings: &BoidSettings) -> Vector2 {
    if boid_settings.mouse_force == 0.0 {
        return Vector2::ZERO;
    }
    let mut diff = boid_settings.mouse_position - position;
    let sqr_diff = diff.x * diff.x + diff.y * diff.y;
    if sqr_diff < boid_settings.sqr_mouse_range {
        // Squared reppel force
        if boid_settings.mouse_force < 0.0 {
            let norm_diff = f32::sqrt(sqr_diff);
            diff.x *= (1.0 - sqr_diff / boid_settings.sqr_mouse_range) / norm_diff
                * boid_settings.mouse_force;
            diff.y *= (1.0 - sqr_diff / boid_settings.sqr_mouse_range) / norm_diff
                * boid_settings.mouse_force;
        } else if boid_settings.mouse_force > 0.0 {
            diff.x *= (1.0 / boid_settings.mouse_range) * boid_settings.mouse_force;
            diff.y *= (1.0 / boid_settings.mouse_range) * boid_settings.mouse_force;
        }
        diff
    } else {
        Vector2::ZERO
    }
}

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

fn wrapping(position: &mut Vector2, boid_settings: &BoidSettings) {
    if let BorderSettings::Wrapping
    | BorderSettings::BoundedVertical {
        turn_force: _,
        margin: _,
    } = boid_settings.border_settings
    {
        position.x = position.x.rem_euclid(boid_settings.width as f32);
    }

    if let BorderSettings::Wrapping
    | BorderSettings::BoundedHorizontal {
        turn_force: _,
        margin: _,
    } = boid_settings.border_settings
    {
        position.y = position.y.rem_euclid(boid_settings.height as f32);
    }
}

fn boid_rules(
    index: usize,
    grid: &Grid<super::Boid>,
    boid_settings: &BoidSettings,
    prev_index: &mut i32,
) -> Vector2 {
    let mut avg = Vector2::ZERO;
    let mut align = Vector2::ZERO;
    let mut vis_count: u16 = 0;
    let mut sep = Vector2::ZERO;
    let mut prot_count: u16 = 0;
    let mut prev_found = false;

    let width = boid_settings.width;
    let height = boid_settings.height;
    let boid = &grid.values[index].val;
    let position = boid.position;

    let grid_column = (position.x / width as f32 * grid.columns as f32) as i32 - CELLS_IN_RADIUS;
    let grid_row = (position.y / height as f32 * grid.rows as f32) as i32 - CELLS_IN_RADIUS;
    const LOCAL_GRID_WIDTH: usize = CELLS_IN_RADIUS as usize * 2 + 1;
    const LOCAL_GRID_SIZE: usize = LOCAL_GRID_WIDTH * LOCAL_GRID_WIDTH;
    let mut bins = [0.0; LOCAL_GRID_SIZE];
    let mut indices = [0; LOCAL_GRID_SIZE];

    for r_offset in 0..LOCAL_GRID_WIDTH as i32 {
        let other_row = grid_row + r_offset;
        for c_offset in 0..LOCAL_GRID_WIDTH as i32 {
            let other_column = grid_column + c_offset;
            let i = (c_offset + r_offset * LOCAL_GRID_WIDTH as i32) as usize;
            indices[i] = grid.index_from_pos(other_row, other_column);
            bins[i] = if let Some(grid_node) = grid.get_grid_node(other_row, other_column) {
                grid_node.count as f32
            } else {
                0.0
            } + if i == 0 { 0.0 } else { bins[i - 1] };
        }
    }

    let increment = (bins[LOCAL_GRID_SIZE - 1] / MAX_SAMPLES as f32).max(1.0);
    let mut acc = 0.0;
    let current_group = boid.group;
    let mut total = 0;
    for current_bin in 0..LOCAL_GRID_SIZE {
        let cell_index = indices[current_bin];
        let mut local_prev_index = -1;
        for boid_index in grid.iter_from_index(cell_index) {
            if acc >= bins[current_bin] && (current_bin != LOCAL_GRID_SIZE / 2 || prev_found) {
                break;
            }
            total += 1;

            if boid_index == index {
                prev_found = true;
                acc += increment;
                if local_prev_index != -1 {
                    *prev_index = local_prev_index;
                }
                continue;
            }

            local_prev_index = boid_index as i32;
            let other_boid = grid.get_val(boid_index);
            let other_position = other_boid.position;
            let x_diff = other_position.x - position.x;
            let y_diff = other_position.y - position.y;
            let distance = x_diff * x_diff + y_diff * y_diff;
            if distance < boid_settings.sqr_protected_range {
                sep.x -= x_diff;
                sep.y -= y_diff;
                prot_count += 1;
            } else if distance < boid_settings.sqr_visible_range
                && other_boid.group == current_group
            {
                avg.x += x_diff;
                avg.y += y_diff;
                align = align + other_boid.velocity;
                vis_count += 1;
            }
            acc += increment;
        }
    }

    if index == 0 {
        eprintln!("{}", total);
    }
    if prot_count > 0 {
        sep.x /= prot_count as f32;
        sep.y /= prot_count as f32;
    }

    if vis_count > 0 {
        avg.x /= vis_count as f32;
        avg.y /= vis_count as f32;
        align.x /= vis_count as f32;
        align.y /= vis_count as f32;
    }

    let mut accel = Vector2::ZERO;
    accel.x += avg.x * 0.01 + align.x * 0.05 + sep.x * 0.05;
    accel.y += avg.y * 0.01 + align.y * 0.05 + sep.y * 0.05;
    accel
}

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
    let mut prev_index: i32 = -1;

    let mut accel = boid_rules(index, grid, boid_settings, &mut prev_index);

    // Gravity
    accel.y += boid_settings.gravity;

    // Noise
    accel = accel + rand_diffuse(boid_settings, delta);

    // Air Resistance
    accel = accel - drag(velocity, boid_settings);

    // Mouse force
    accel = accel + mouse_force(position, boid_settings);

    // Force on screen
    accel = accel + border_force(position, velocity, boid_settings);

    let boid = &mut grid.values[index].val;
    // Update velocity based on differentials.
    let mut velocity = boid.velocity;
    velocity.x += accel.x * delta;
    velocity.y += accel.y * delta;

    // Clipping.
    let speed = velocity.magnitude();
    if speed < boid_settings.min_speed && speed != 0.0 {
        let ratio = boid_settings.min_speed / speed;
        velocity.x *= ratio;
        velocity.y *= ratio;
    }

    // Update position based on velocity.
    let mut new_position = boid.position;
    new_position.x += velocity.x * delta;
    new_position.y += velocity.y * delta;
    wrapping(&mut new_position, boid_settings);
    boid.velocity = velocity;
    boid.position = new_position;

    // Update grid's linked list
    let width = boid_settings.width;
    let height = boid_settings.height;
    let grid_column = (position.x / width as f32 * grid.columns as f32) as i32;
    let grid_row = (position.y / height as f32 * grid.rows as f32) as i32;
    let new_grid_column = (new_position.x / width as f32 * grid.columns as f32) as i32;
    let new_grid_row = (new_position.y / height as f32 * grid.rows as f32) as i32;

    let grid_index = grid.index_from_pos(grid_row, grid_column);
    if grid_index >= 0 {
        let grid_index = grid_index as usize;
        let next_index = grid.values[index].next_index;
        let grid_node = &mut grid.grid[grid_index];
        // Current boid is first
        if prev_index == -1 {
            grid_node.first = next_index;
        } else {
            // Other boids before in grid.
            grid.values[prev_index as usize].next_index = next_index;
        }

        if grid_node.last == index as i32 {
            grid_node.last = prev_index;
        }
        grid_node.count -= 1;
    }

    let grid_index = grid.index_from_pos(new_grid_row, new_grid_column);
    if grid_index >= 0 {
        let grid_index = grid_index as usize;
        grid.values[index].next_index = -1;
        let last_index = grid.grid[grid_index].last;
        if last_index >= 0 {
            grid.values[last_index as usize].next_index = index as i32;
        } else {
            grid.grid[grid_index].first = index as i32;
        }
        grid.grid[grid_index].last = index as i32;
        grid.grid[grid_index].count += 1;
    }
}
