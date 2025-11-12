use crate::grid::{Grid, ValueNode};
use crate::vector2::Vector2;
use fastrand;

const GRID_MODIFIER: i32 = 2;
const MAX_SAMPLES: i32 = 300;

pub enum BorderSettings {
    None,
    Bounded { turn_force: f32, margin: f32 },
    BoundedVertical { turn_force: f32, margin: f32 },
    BoundedHorizontal { turn_force: f32, margin: f32 },
    Wrapping,
}

pub struct BoidSettings {
    // Basic settings
    pub protected_range: f32,
    pub visible_range: f32,
    // Window settings
    pub width: usize,
    pub height: usize,
    // Border
    pub border_settings: BorderSettings,
    // Gravity
    pub gravity: f32,
    // Noise
    pub noise_force: Option<f32>,
    // Min Speed
    pub min_speed: f32,
    // Friction
    pub friction_coefficient: f32,
    pub squared_friction: bool,
    // Mouse
    pub mouse_force: f32,
    pub mouse_range: f32,
    pub mouse_position: Vector2,
    // Pre-calculations
    sqr_protected_range: f32,
    sqr_visible_range: f32,
    sqr_mouse_range: f32,
}

impl BoidSettings {
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

    pub fn update_window(&mut self, width: usize, height: usize) -> &mut Self {
        self.width = width;
        self.height = height;
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

#[derive(Debug, Copy, Clone)]
pub struct Boid {
    pub position: Vector2,
    pub velocity: Vector2,
    group: u32,
}

pub fn populate(count: usize, group_count: u32, boid_settings: &BoidSettings) -> Grid<Boid> {
    let mut generator = fastrand::Rng::new();
    let grid_columns = ((GRID_MODIFIER as f32 * boid_settings.width as f32
        / boid_settings
            .visible_range
            .max(boid_settings.protected_range)) as usize)
        .max(1);
    let grid_rows = ((GRID_MODIFIER as f32 * boid_settings.height as f32
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
pub fn resize_grid(grid: &mut Grid<Boid>, boid_settings: &BoidSettings) -> () {
    let grid_columns = ((GRID_MODIFIER as f32 * boid_settings.width as f32
        / boid_settings
            .visible_range
            .max(boid_settings.protected_range)) as usize)
        .max(1);
    let grid_rows = ((GRID_MODIFIER as f32 * boid_settings.height as f32
        / boid_settings
            .visible_range
            .max(boid_settings.protected_range)) as usize)
        .max(1);
    let mut new_grid: Grid<Boid> = Grid::new(grid.count, grid_columns, grid_rows);
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
        new_grid.add_val(boid.clone(), grid_column, grid_row);
    }
    *grid = new_grid;
}

fn drag(velocity: Vector2, boid_settings: &BoidSettings) -> Vector2 {
    let k = boid_settings.friction_coefficient;
    if boid_settings.squared_friction {
        let x = velocity.x.signum() * velocity.x * velocity.x * k;
        let y = velocity.y.signum() * velocity.y * velocity.y * k;
        Vector2 { x, y }
    } else {
        Vector2 {
            x: velocity.x * k,
            y: velocity.y * k,
        }
    }
}

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

fn mouse_force(position: Vector2, boid_settings: &BoidSettings) -> Vector2 {
    if boid_settings.mouse_force == 0.0 {
        return Vector2::ZERO;
    }
    let mut diff = boid_settings.mouse_position - position;
    let sqr_diff = diff.x * diff.x + diff.y * diff.y;
    // Squared reppel force
    if sqr_diff < boid_settings.sqr_mouse_range {
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
    grid: &Grid<Boid>,
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

    let grid_column = (position.x / width as f32 * grid.columns as f32) as i32 - GRID_MODIFIER;
    let grid_row = (position.y / height as f32 * grid.rows as f32) as i32 - GRID_MODIFIER;
    const LOCAL_GRID_WIDTH: usize = GRID_MODIFIER as usize * 2 + 1;
    const LOCAL_GRID_SIZE: usize = LOCAL_GRID_WIDTH * LOCAL_GRID_WIDTH;
    let mut bins = [0.0; LOCAL_GRID_SIZE];
    let mut indices = [0; LOCAL_GRID_SIZE];
    for r_offset in 0..LOCAL_GRID_WIDTH as i32 {
        let other_row = grid_row + r_offset;
        if other_row < 0 || other_row >= grid.rows as i32 {
            continue;
        }
        for c_offset in 0..LOCAL_GRID_WIDTH as i32 {
            let other_column = grid_column + c_offset;
            if other_column < 0 || other_column >= grid.columns as i32 {
                continue;
            }
            let grid_node = grid.get_grid_node(other_column as usize, other_row as usize);
            let mut other_index = grid_node.first;
            if other_index == index as i32 {
                prev_found = true;
                other_index = grid.values[index].next_index;
            }
            let i = (c_offset + r_offset * LOCAL_GRID_WIDTH as i32) as usize;
            indices[i] = other_index;
            bins[i] = if i == 0 {
                grid_node.count as f32
            } else {
                grid_node.count as f32 + bins[i - 1]
            };
        }
    }
    // Store the grid nodes, increment a counter bij total / 200, choose index to progress in based on this value.
    let increment = (bins[LOCAL_GRID_SIZE - 1] / MAX_SAMPLES as f32).max(1.0);
    let mut acc = 0.0;
    let current_group = boid.group;
    for current_bin in 0..LOCAL_GRID_SIZE {
        let mut other_index = indices[current_bin];
        while other_index >= 0 && acc < bins[current_bin] {
            let other_elem = &grid.values[other_index as usize];
            let other_boid = other_elem.val;
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

            if other_elem.next_index == index as i32 {
                *prev_index = other_index;
                prev_found = true;
                other_index = grid.values[index].next_index;
            } else {
                other_index = other_elem.next_index;
            }
            acc += increment;
        }
        // Find the previous boid in the grid (no longer matters if MAX_SAMPLES > LOCAL_GRID_SIZE)
        if current_bin == LOCAL_GRID_SIZE / 2 && !prev_found {
            while other_index >= 0 {
                let other_elem = &grid.values[other_index as usize];
                if other_elem.next_index == index as i32 {
                    *prev_index = other_index;
                    break;
                } else {
                    other_index = other_elem.next_index;
                }
            }
        }
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

fn update_boid(index: usize, grid: &mut Grid<Boid>, boid_settings: &BoidSettings, delta: f32) {
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

    if grid_row >= 0
        && grid_row < grid.rows as i32
        && grid_column >= 0
        && grid_column < grid.columns as i32
    {
        let next_index = grid.values[index].next_index;
        let grid_node = &mut grid.grid[grid_column as usize + grid_row as usize * grid.columns];
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
        grid.grid[grid_column as usize + grid_row as usize * grid.columns].count -= 1;
    }

    if new_grid_row >= 0
        && new_grid_row < grid.rows as i32
        && new_grid_column >= 0
        && new_grid_column < grid.columns as i32
    {
        let new_grid_index = new_grid_column as usize + new_grid_row as usize * grid.columns;
        grid.values[index].next_index = -1;
        let last_index = grid.grid[new_grid_index].last;
        if last_index != -1 {
            grid.values[last_index as usize].next_index = index as i32;
        } else {
            grid.grid[new_grid_index].first = index as i32;
        }
        grid.grid[new_grid_index].last = index as i32;
        grid.grid[new_grid_column as usize + new_grid_row as usize * grid.columns].count += 1;
    }
}

pub fn update_boids(grid: &mut Grid<Boid>, boid_settings: &BoidSettings, delta: f32) {
    let boid_count = grid.values.len();

    for i in 0..boid_count {
        update_boid(i, grid, boid_settings, delta);
    }
}
