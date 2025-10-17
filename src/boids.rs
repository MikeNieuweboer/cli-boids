// TODO: Gravity
use crate::vector2::Vector2;
use fastrand;

pub struct BoidSettings {
    pub protected_range: f64,
    pub visible_range: f64,
    pub width: usize,
    pub height: usize,
    pub min_speed: f64,
    pub max_speed: f64,
    pub margin: f64,
    sqr_protected_range: f64,
    sqr_visible_range: f64,
    sqr_min_speed: f64,
    sqr_max_speed: f64,
}

impl BoidSettings {
    pub fn new(
        protected_range: f64,
        visible_range: f64,
        width: usize,
        height: usize,
        min_speed: f64,
        max_speed: f64,
        margin: f64,
    ) -> BoidSettings {
        BoidSettings {
            protected_range,
            visible_range,
            width,
            height,
            min_speed,
            max_speed,
            margin,
            sqr_protected_range: protected_range * protected_range,
            sqr_visible_range: visible_range * visible_range,
            sqr_min_speed: min_speed * min_speed,
            sqr_max_speed: max_speed * max_speed,
        }
    }
}

#[derive(Debug)]
pub struct Boid {
    pub position: Vector2,
    pub velocity: Vector2,
}

pub fn populate(count: usize, boid_settings: &BoidSettings) -> Vec<Boid> {
    let mut boids: Vec<Boid> = Vec::with_capacity(count);
    let mut generator = fastrand::Rng::new();

    let width = boid_settings.width;
    let height = boid_settings.height;
    let velocity = Vector2 { x: 0f64, y: 0f64 };
    for _ in 0..count {
        let position = Vector2 {
            x: generator.f64() * (width as f64),
            y: generator.f64() * (height as f64),
        };
        boids.push(Boid { position, velocity });
    }
    boids
}

fn update_boid(index: usize, boids: &mut [Boid], boid_settings: &BoidSettings, delta: f64) {
    // Basic boid forces
    let position = boids[index].position;
    let velocity = boids[index].velocity;
    let mut avg = Vector2::ZERO;
    let mut align = Vector2::ZERO;
    let mut vis_count: u16 = 0;
    let mut sep = Vector2::ZERO;
    let mut prot_count: u16 = 0;
    for other in boids.iter() {
        let other_position = other.position;
        let x_diff = other_position.x - position.x;
        let y_diff = other_position.y - position.y;
        let distance = x_diff * x_diff + y_diff * y_diff;
        if distance < boid_settings.sqr_protected_range {
            sep.x -= x_diff;
            sep.y -= y_diff;
            prot_count += 1;
        } else if distance < boid_settings.sqr_visible_range {
            avg.x += x_diff;
            avg.y += y_diff;
            align = align + other.velocity;
            vis_count += 1;
        }
    }
    if prot_count > 0 {
        sep.x /= prot_count as f64;
        sep.y /= prot_count as f64;
    }

    if vis_count > 0 {
        avg.x /= vis_count as f64;
        avg.y /= vis_count as f64;
        align.x /= vis_count as f64;
        align.y /= vis_count as f64;
    }

    let mut accel = Vector2::ZERO;
    accel.x += avg.x * 0.005 + align.x * 0.05 + sep.x * 0.05;
    accel.y += avg.y * 0.005 + align.y * 0.05 + sep.y * 0.05;

    // Force on screen
    let margin = boid_settings.margin;
    let turn_force = boid_settings.sqr_max_speed / margin;
    if position.x < margin {
        accel.x += turn_force;
        accel.y += velocity.y.signum() * turn_force * 0.01;
    } else if position.x > (boid_settings.width as f64 - margin) {
        accel.x -= turn_force;
        accel.y += velocity.y.signum() * turn_force * 0.01;
    }

    if position.y < margin {
        accel.y += turn_force;
        accel.x += velocity.x.signum() * turn_force * 0.01;
    } else if position.y > (boid_settings.height as f64 - margin) {
        accel.y -= turn_force;
        accel.x += velocity.x.signum() * turn_force * 0.01;
    }

    let boid = &mut boids[index];
    // Update velocity based on differentials.
    let mut velocity = boid.velocity;
    velocity.x += accel.x * delta;
    velocity.y += accel.y * delta;

    // Clipping.
    let speed = velocity.magnitude();
    if speed == 0f64 {
        velocity.x = boid_settings.min_speed;
    } else if speed < boid_settings.min_speed {
        let ratio = boid_settings.min_speed / speed;
        velocity.x *= ratio;
        velocity.y *= ratio;
    } else if speed > boid_settings.max_speed {
        let ratio = boid_settings.max_speed / speed;
        velocity.x *= ratio;
        velocity.y *= ratio;
    }

    // Update position based on velocity.
    let mut new_position = boid.position;
    new_position.x += velocity.x * delta;
    new_position.y += velocity.y * delta;

    boid.velocity = velocity;
    boid.position = new_position;
}

pub fn update_boids(boids: &mut Vec<Boid>, boid_settings: &BoidSettings, delta: f64) -> () {
    let boid_count = boids.len();

    for i in 0..boid_count {
        update_boid(i, boids, boid_settings, delta);
    }
}
