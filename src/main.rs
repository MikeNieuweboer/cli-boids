use crossterm::{
    cursor::{Hide, Show},
    event::{
        DisableFocusChange, DisableMouseCapture, EnableFocusChange, EnableMouseCapture, Event,
        KeyCode, KeyModifiers, MouseButton, MouseEvent, MouseEventKind, poll, read,
    },
    execute, queue,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode, window_size,
    },
};
use std::{
    io::{Result, Write, stdout},
    thread::sleep,
    time::{Duration, Instant},
};

mod boids;
mod grid;
mod menu;
mod render;
mod vector2;

use crate::boids::{Boid, BoidSettings, BorderSettings, populate, update_boids};
use crate::grid::Grid;
use crate::render::draw_boids;

const COUNT: usize = 2000;
const FRAME_TIME: Duration = Duration::from_millis(20);

const SEPERATION_DIST: f32 = 2f32;
const COHESION_DIST: f32 = 5f32;
const MIN_SPEED: f32 = 2.0;
const TURN_FORCE: f32 = 1.5;
const MARGIN: f32 = 20.0;
const GRAVITY: f32 = 0.08;
const NOISE_FORCE: f32 = 0.05;
const FRICTION_COEFFICIENT: f32 = 0.01;
const SQUARED_FRICTION: bool = true;
const MOUSE_RANGE: f32 = 20.0;
const MOUSE_FORCE: f32 = 5.0;
const MOUSE_RANGE_DOWN: f32 = 10.0;
const MOUSE_FORCE_DOWN: f32 = -5.0;

struct SimulationSettings {
    paused: bool,
    running: bool,
    frame_time: Duration,
}

impl SimulationSettings {
    pub const fn init() -> SimulationSettings {
        SimulationSettings {
            paused: false,
            running: true,
            frame_time: FRAME_TIME,
        }
    }
}

fn settings_init() -> Result<BoidSettings> {
    let size = window_size()?;
    let height = (size.rows * 2u16) as usize;
    let width = size.columns as usize;

    let mut boid_settings = BoidSettings::new(SEPERATION_DIST, COHESION_DIST, width, height);
    boid_settings
        .set_gravity(GRAVITY)
        .set_min_speed(MIN_SPEED)
        .set_border(BorderSettings::Bounded {
            turn_force: TURN_FORCE,
            margin: MARGIN,
        })
        .set_noise(NOISE_FORCE)
        .set_friction(FRICTION_COEFFICIENT, SQUARED_FRICTION)
        .set_mouse_force(MOUSE_FORCE, MOUSE_RANGE);
    Ok(boid_settings)
}

#[inline(always)]
fn quit(sim_settings: &mut SimulationSettings) {
    sim_settings.running = false;
}

fn pause(sim_settings: &mut SimulationSettings) -> Result<()> {
    let mut stdout = stdout();
    if sim_settings.paused {
        sim_settings.paused = false;
        execute!(stdout, EnableMouseCapture)?;
    } else {
        sim_settings.paused = true;
        execute!(stdout, DisableMouseCapture)?;
    }
    Ok(())
}

fn on_mouse_event(event: MouseEvent, boid_settings: &mut BoidSettings) {
    match event.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            boid_settings.set_mouse_force(MOUSE_FORCE_DOWN, MOUSE_RANGE_DOWN);
        }
        MouseEventKind::Up(MouseButton::Left) => {
            boid_settings.set_mouse_force(MOUSE_FORCE, MOUSE_RANGE);
        }
        _ => (),
    }
    boid_settings.set_mouse_position(event.column as f32 + 0.5, event.row as f32 * 2.0 + 1.0);
}

fn on_resize(
    new_columns: usize,
    new_rows: usize,
    boid_data: &mut Grid<Boid>,
    boid_settings: &mut BoidSettings,
) {
    boid_settings.update_window(new_columns, new_rows * 2, boid_data);
}

fn handle_input(
    sim_settings: &mut SimulationSettings,
    boid_settings: &mut BoidSettings,
    boid_data: &mut Grid<Boid>,
) -> Result<()> {
    while poll(Duration::from_millis(0))? {
        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Esc => quit(sim_settings),
                KeyCode::Char(' ') => pause(sim_settings)?,
                KeyCode::Char('q') => quit(sim_settings),
                KeyCode::Char('c') => {
                    if event.modifiers.contains(KeyModifiers::CONTROL) {
                        quit(sim_settings);
                    }
                }
                _ => (),
            },
            Event::Mouse(event) => on_mouse_event(event, boid_settings),
            Event::FocusGained => {
                // Regain mouse control
                boid_settings.set_mouse_force(MOUSE_FORCE, MOUSE_RANGE);
            }
            Event::FocusLost => {
                // Lose mouse control
                boid_settings.set_mouse_force(0.0, 0.0);
            }
            Event::Resize(c, r) => on_resize(c as usize, r as usize, boid_data, boid_settings),
            _ => (),
        }
    }
    Ok(())
}

fn sim_delay(start: Instant, sim_settings: &SimulationSettings) -> f32 {
    let current_frame_time = start.elapsed();
    if current_frame_time.as_millis() < sim_settings.frame_time.as_millis() {
        sleep(FRAME_TIME.abs_diff(current_frame_time));
        FRAME_TIME.as_millis() as f32 / 1000.0
    } else {
        current_frame_time.as_millis() as f32 / 1000.0
    }
}

fn simulate(
    mut sim_settings: SimulationSettings,
    mut boid_data: Grid<Boid>,
    boid_settings: &mut BoidSettings,
) -> Result<()> {
    let mut stdout = stdout();
    let mut last_duration: f32 = 0.02;
    while sim_settings.running {
        let now = Instant::now();
        let size = window_size()?;

        // Poll for any input and execute the corresponding action
        handle_input(&mut sim_settings, boid_settings, &mut boid_data)?;

        if sim_settings.paused {
            continue;
        }
        queue!(stdout, Clear(ClearType::All))?;

        const TIME_SCALE: f32 = 10.0;
        update_boids(&mut boid_data, boid_settings, last_duration * TIME_SCALE);

        draw_boids(&mut stdout, &boid_data.values, &size, boid_settings)?;

        stdout.flush()?;

        // Delay the next frame based on target frame rate.
        last_duration = sim_delay(now, &sim_settings);
    }
    Ok(())
}

fn start() -> Result<()> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(
        stdout,
        EnterAlternateScreen,
        Clear(ClearType::All),
        Hide,
        EnableMouseCapture,
        EnableFocusChange,
    )?;

    let mut boid_settings = settings_init()?;
    let boid_data: Grid<Boid> = populate(COUNT, 2, &boid_settings);
    let sim_settings = SimulationSettings::init();
    simulate(sim_settings, boid_data, &mut boid_settings)?;

    execute!(
        stdout,
        LeaveAlternateScreen,
        Show,
        DisableMouseCapture,
        DisableFocusChange
    )?;
    disable_raw_mode()?;

    Ok(())
}

fn main() -> Result<()> {
    start()
}
