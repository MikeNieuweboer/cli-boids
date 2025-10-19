use crate::vector2::Vector2;
use fastrand;

pub struct BoidSettings {
    // Basic settings
    pub protected_range: f64,
    pub visible_range: f64,
    // Window settings
    pub width: usize,
    pub height: usize,
    // Border
    pub turn_force: f64,
    pub margin: f64,
    // Gravity
    pub gravity: f64,
    // Noise
    pub noise_force: f64,
    // Min Speed
    pub min_speed: f64,
    // Friction
    pub friction_coefficient: f64,
    pub squared_friction: bool,
    // Mouse
    pub mouse_force: f64,
    pub mouse_range: f64,
    pub mouse_position: Vector2,
    // Pre-calculations
    sqr_protected_range: f64,
    sqr_visible_range: f64,
    sqr_mouse_range: f64,
}

impl BoidSettings {
    pub fn new(
        protected_range: f64,
        visible_range: f64,
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
            margin: 0.0,
            turn_force: 0.0,
            gravity: 0.0,
            min_speed: 0.0,
            noise_force: 0.0,
            friction_coefficient: 0.0,
            squared_friction: false,
            mouse_force: 0.0,
            mouse_range: 0.0,
            sqr_mouse_range: 0.0,
            mouse_position: Vector2::ZERO,
        }
    }

    pub fn update_window(&mut self, width: usize, height: usize) -> &mut Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn set_gravity(&mut self, gravity: f64) -> &mut Self {
        self.gravity = gravity;
        self
    }

    pub fn set_border(&mut self, margin: f64, turn_force: f64) -> &mut Self {
        self.turn_force = turn_force;
        self.margin = margin;
        self
    }

    pub fn set_min_speed(&mut self, min_speed: f64) -> &mut Self {
        self.min_speed = min_speed;
        self
    }

    pub fn set_noise(&mut self, force: f64) -> &mut Self {
        self.noise_force = force;
        self
    }

    pub fn set_friction(&mut self, friction_coefficient: f64, squared_friction: bool) -> &mut Self {
        self.friction_coefficient = friction_coefficient;
        self.squared_friction = squared_friction;
        self
    }

    pub fn set_mouse_force(&mut self, mouse_force: f64, mouse_range: f64) -> &mut Self {
        self.mouse_force = mouse_force;
        self.mouse_range = mouse_range;
        self.sqr_mouse_range = mouse_range * mouse_range;
        self
    }

    pub fn set_mouse_position(&mut self, x: f64, y: f64) -> &mut Self {
        self.mouse_position = Vector2 { x, y };
        self
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

fn rand_diffuse(boid_settings: &BoidSettings, delta: f64) -> Vector2 {
    let diffuse = f64::sqrt(delta);
    let force = boid_settings.noise_force;
    Vector2 {
        x: force * (fastrand::f64() - 0.5) / diffuse,
        y: force * (fastrand::f64() - 0.5) / diffuse,
    }
}

fn mouse_force(position: Vector2, boid_settings: &BoidSettings) -> Vector2 {
    let mut diff = boid_settings.mouse_position - position;
    let sqr_diff = diff.x * diff.x + diff.y * diff.y;
    // Squared reppel force
    if sqr_diff < boid_settings.sqr_mouse_range {
        if boid_settings.mouse_force < 0.0 {
            diff.x *= (1.0 - sqr_diff / boid_settings.sqr_mouse_range) * boid_settings.mouse_force;
            diff.y *= (1.0 - sqr_diff / boid_settings.sqr_mouse_range) * boid_settings.mouse_force;
        } else if boid_settings.mouse_force > 0.0 {
            diff.x *= boid_settings.mouse_force;
            diff.y *= boid_settings.mouse_force;
        }
        diff
    } else {
        Vector2::ZERO
    }
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
    if prot_count > 1 {
        prot_count -= 1;
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

    // Gravity
    accel.y += boid_settings.gravity;

    // Noise
    accel = accel + rand_diffuse(boid_settings, delta);

    // Air Resistance
    accel = accel - drag(velocity, boid_settings);

    // Mouse force
    accel = accel + mouse_force(position, boid_settings);

    // Force on screen
    let margin = boid_settings.margin;
    let turn_force = boid_settings.turn_force;
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
    if speed < 3.0 && speed != 0.0 {
        let ratio = 3.0 / speed;
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
