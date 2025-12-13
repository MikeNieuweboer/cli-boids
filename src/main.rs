//! # cli-boids
//!
//! Contains the top level logic for running and displaying the boid simulation.
//! This includes the setting up of an alternate screen using crossterm and
//! handling the input from the user, along with calls to the [`boids`] and [`render`]
//! modules for simulating and showing the boids.

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{
        DisableFocusChange, DisableMouseCapture, EnableFocusChange, EnableMouseCapture, Event,
        KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind, poll, read,
    },
    execute, queue,
    style::Print,
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

use crate::render::draw_boids;
use crate::{
    boids::{Boid, BoidSettings, BorderSettings, populate, update_boids},
    menu::Menu,
};
use crate::{grid::Grid, menu::MenuItem};

// Simulation settings
const COUNT: usize = 5000;
const GROUP_COUNT: u8 = 2;
const FRAME_TIME: Duration = Duration::from_millis(20);

// Boid settings
const SEPERATION_DIST: f32 = 2f32;
const COHESION_DIST: f32 = 5f32;
const COHESION_FORCE: f32 = 0.01f32;
const SEPARATION_FORCE: f32 = 0.05f32;
const ALIGNMENT_FORCE: f32 = 0.05f32;
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

enum MenuID {
    SeperationDistance,
    CohesionForce,
}

/// Settings related to running the simulations, unlike
/// [`BoidSettings`], which controls the behavior of the
/// simulated boids.
struct SimulationSettings {
    /// Whether the main simulation loop should be running.
    running: bool,

    /// Whether the main simulation loop is paused.
    paused: bool,

    /// The target interval between frames, can be exceeded if the simulation is
    /// too intensive.
    frame_time: Duration,
}

impl SimulationSettings {
    // TODO: Replace with new() for configurable settings.
    /// Initialises a new [`SimulationSettings`] struct with the values
    /// required at the start of the simulation loop.
    pub const fn init() -> SimulationSettings {
        SimulationSettings {
            paused: false,
            running: true,
            frame_time: FRAME_TIME,
        }
    }
}

/// Initialises [`BoidSettings`] for the simulation based on the global defines.
///
/// ## TODO
/// Must be replaced by an actual settings manager.
///
/// # Errors
///
/// This function will return an error if it fails to interact with the terminal.
fn boid_settings_init() -> Result<BoidSettings> {
    let size = window_size()?;
    let height = (size.rows * 2u16) as usize;
    let width = size.columns as usize;

    let mut boid_settings = BoidSettings::new(
        SEPERATION_DIST,
        COHESION_DIST,
        COHESION_FORCE,
        SEPARATION_FORCE,
        ALIGNMENT_FORCE,
        width,
        height,
    );
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

/// Sets the `sim_settings` to quit the main simulation loop.
#[inline(always)]
fn quit(sim_settings: &mut SimulationSettings) {
    sim_settings.running = false;
}

/// Sets the `sim_settings` to switch from pause to unpause and vice versa. Also
/// enables or disables mouse capture with the pause and unpause respectively.
///
/// # Errors
///
/// This function will return an error if it fails to interact with the terminal.
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

/// Handles key related input `event`s.
///
/// # Errors
///
/// This function will return an error if it fails to interact with the terminal.
fn on_key_event(event: KeyEvent, sim_settings: &mut SimulationSettings) -> Result<()> {
    match event.code {
        KeyCode::Esc => quit(sim_settings),
        KeyCode::Char(' ') => pause(sim_settings)?,
        KeyCode::Char('q') => quit(sim_settings),
        KeyCode::Char('c') => {
            if event.modifiers.contains(KeyModifiers::CONTROL) {
                quit(sim_settings);
            }
        }
        _ => (),
    };
    Ok(())
}

/// Handles mouse related input `event`s.
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
    // Set mouse position to middle of character
    boid_settings.set_mouse_position(event.column as f32 + 0.5, event.row as f32 * 2.0 + 1.0);
}

/// Handles the logic for when the terminal window is resized.
#[inline(always)]
fn on_resize(
    new_columns: usize,
    new_rows: usize,
    boid_data: &mut Grid<Boid>,
    boid_settings: &mut BoidSettings,
) {
    boid_settings.update_window(new_columns, new_rows * 2, boid_data);
}

fn on_menu_change<'a>(changed_item: &MenuItem<'a, MenuID>, boid_settings: &mut BoidSettings) {
    match changed_item {
        MenuItem::IntSlider { id, current, .. } => match id {
            _ => (),
        },
        MenuItem::FloatSlider { id, current, .. } => match id {
            MenuID::SeperationDistance => boid_settings.protected_range = *current,
            MenuID::CohesionForce => boid_settings.cohesion = *current,
        },
        MenuItem::Toggle { id, current } => match id {
            _ => (),
        },
        MenuItem::Choice { id, current, .. } => match id {
            _ => (),
        },
    }
}

/// Reads and handles all the input currently in the queue.
///
/// # Errors
///
/// This function will return an error if it fails to interact with the
/// terminal.
fn handle_input<'a>(
    sim_settings: &mut SimulationSettings,
    boid_settings: &mut BoidSettings,
    boid_data: &mut Grid<Boid>,
    menu: &mut Menu<'a, MenuID>,
) -> Result<()> {
    while poll(Duration::from_millis(0))? {
        let event = read()?;
        match event {
            Event::Key(key_event) => on_key_event(key_event, sim_settings)?,
            Event::Mouse(mouse_event) => on_mouse_event(mouse_event, boid_settings),
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
        if let Some(changed_item) = menu::handle_input(menu, &event) {
            on_menu_change(changed_item, boid_settings);
        }
    }
    Ok(())
}

/// Enforces a minimum interval between frames by sleeping if
/// the difference between `start` and now is smaller than the frame time
/// set in the `sim_settings`.
fn sim_delay(start: Instant, sim_settings: &SimulationSettings) -> f32 {
    let current_frame_time = start.elapsed();
    if current_frame_time.as_millis() < sim_settings.frame_time.as_millis() {
        sleep(FRAME_TIME.abs_diff(current_frame_time));
        FRAME_TIME.as_millis() as f32 / 1000.0
    } else {
        current_frame_time.as_millis() as f32 / 1000.0
    }
}

/// Performs the main simulation loop of the boids.
/// This involves the handling of input, updating of the boids
/// and rendering them to the terminal.
///
/// # Errors
///
/// This function will return an error if the simulation fails to manipulate
/// the terminal.
fn simulate<'a>(
    mut sim_settings: SimulationSettings,
    mut boid_data: Grid<Boid>,
    mut menu: Menu<'a, MenuID>,
    boid_settings: &mut BoidSettings,
) -> Result<()> {
    let mut stdout = stdout();
    let mut last_duration: f32 = 0.02;
    while sim_settings.running {
        let now = Instant::now();
        let size = window_size()?;

        // Poll for any input and execute the corresponding action
        handle_input(&mut sim_settings, boid_settings, &mut boid_data, &mut menu)?;

        if sim_settings.paused {
            continue;
        }

        queue!(stdout, Clear(ClearType::All))?;

        // TODO: remove the need for this timescale by using sane parameters.
        const TIME_SCALE: f32 = 10.0;
        update_boids(&mut boid_data, boid_settings, last_duration * TIME_SCALE);

        draw_boids(&mut stdout, boid_data.iter_all(), &size, boid_settings)?;
        queue!(stdout, MoveTo(0, 0), Print(last_duration))?;

        // Write the command queue to the terminal.
        stdout.flush()?;

        // Delay the next frame based on target frame rate.
        last_duration = sim_delay(now, &sim_settings);
    }
    Ok(())
}

/// Prepares the terminal for the simulation and input.
/// This is achieved by switching the terminal to an alternate screen and
/// turning on raw mode and capturing the input.
///
/// # Errors
///
/// This function will return an error if it fails to apply the settings to
/// the terminal.
fn prepare_stdout() -> Result<()> {
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
    Ok(())
}

/// Returns the terminal back to its main screen and reverts the settings from
/// [`prepare_stdout`].
///
/// # Errors
///
/// This function will return an error if it fails to revert the terminal.
fn revert_stdout() -> Result<()> {
    let mut stdout = stdout();
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

/// Starts the main boid simulation loop. Before this loop is started, the
/// terminal is set up and afterwards, its reverted to its normal behavior.
///
/// # Errors
///
/// This function will return an error if it catches any of the io errors
/// resulting from terminal manipulation.
fn start() -> Result<()> {
    prepare_stdout()?;

    let mut boid_settings = match boid_settings_init() {
        Ok(settings) => settings,
        Err(e) => {
            revert_stdout()?;
            return Err(e);
        }
    };
    let boid_data: Grid<Boid> = populate(COUNT, GROUP_COUNT, &boid_settings);
    let sim_settings = SimulationSettings::init();
    let menu = Menu::new();
    let result = simulate(sim_settings, boid_data, menu, &mut boid_settings);

    revert_stdout()?;

    result
}

fn main() -> Result<()> {
    start()
}
