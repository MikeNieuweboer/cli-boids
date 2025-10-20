// TODO: Custom input.
// TODO: Change mouse behaviour when clicking.
// TODO: Groups.
// TODO: Border conditions.
// TODO: Some and none for optional settings.
// TODO: Change delta based on time.
// TODO: On resize event change size.
// TODO: OPTIMIZE!
// TODO: 3D boids?

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{
        DisableFocusChange, DisableMouseCapture, EnableFocusChange, EnableMouseCapture, Event,
        KeyCode, KeyEvent, MouseEventKind, poll, read,
    },
    execute, queue,
    style::Print,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, WindowSize, disable_raw_mode,
        enable_raw_mode, window_size,
    },
};
use std::{
    io::{Result, Stdout, Write, stdout},
    thread::sleep,
    time::{Duration, Instant},
};

pub mod boids;
pub mod vector2;

use crate::boids::{Boid, BoidSettings, populate, update_boids};

const COUNT: usize = 200;
const FRAME_TIME: Duration = Duration::from_millis(20);

const SEPERATION_DIST: f64 = 2f64;
const COHESION_DIST: f64 = 10f64;
const MIN_SPEED: f64 = 0.0;
const TURN_FORCE: f64 = 10.5;
const MARGIN: f64 = 20.0;
const GRAVITY: f64 = 0.08;
const NOISE_FORCE: f64 = 0.05;
const FRICTION_COEFFICIENT: f64 = 0.01;
const SQUARED_FRICTION: bool = true;
const MOUSE_RANGE: f64 = 10.0;
const MOUSE_FORCE: f64 = -10.0;

fn pos_to_braille(x_norm: f64, y_norm: f64) -> u8 {
    let mut braille: u8 = 1;
    if y_norm > 0.75 {
        if x_norm <= 0.5 {
            braille <<= 6;
        } else {
            braille <<= 7;
        }
    } else {
        if x_norm >= 0.5 {
            braille <<= 3;
        }
        if y_norm > 0.25 {
            if y_norm < 0.5 {
                braille <<= 1;
            } else {
                braille <<= 2;
            }
        }
    }
    braille
}

fn draw_boids(
    stdout: &mut Stdout,
    boids: &Vec<Boid>,
    window_size: &WindowSize,
    boid_settings: &BoidSettings,
) -> Result<()> {
    let rows = window_size.rows;
    let columns = window_size.columns;

    let mut braille_grid = vec![0u8; (rows as usize) * (columns as usize)];
    let width_ratio: f64 = (columns as f64) / (boid_settings.width as f64);
    let height_ratio: f64 = (rows as f64) / (boid_settings.height as f64);

    for boid in boids {
        let position = boid.position;
        let x = position.x * width_ratio;
        let c = x.floor();
        if c as u16 >= columns || (c as i16) < 0 {
            continue;
        }
        let y = position.y * height_ratio;
        let r = y.floor();
        if r as u16 >= rows || (r as i16) < 0 {
            continue;
        }
        let braille = pos_to_braille(x - c, y - r);
        braille_grid[(c as usize) + (r as usize) * (columns as usize)] |= braille;
    }

    for r in 0usize..(rows as usize) {
        for c in 0usize..(columns as usize) {
            let braille = braille_grid[r * (columns as usize) + c] as u16;
            if braille != 0
                && let Ok(braille_string) = String::from_utf16(&[0x2800 | braille])
            {
                queue!(stdout, MoveTo(c as u16, r as u16), Print(braille_string))?;
            }
        }
    }
    Ok(())
}

fn run() -> Result<()> {
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

    let size = window_size()?;
    let height = (size.rows * 2u16) as usize;
    let width = size.columns as usize;
    let mut boid_settings = BoidSettings::new(SEPERATION_DIST, COHESION_DIST, width, height);
    boid_settings
        .set_gravity(GRAVITY)
        .set_min_speed(MIN_SPEED)
        .set_border(MARGIN, TURN_FORCE)
        .set_noise(NOISE_FORCE)
        .set_friction(FRICTION_COEFFICIENT, SQUARED_FRICTION)
        .set_mouse_force(MOUSE_FORCE, MOUSE_RANGE);
    let mut boid_data = populate(COUNT, &boid_settings);
    'simulation: loop {
        let now = Instant::now();
        let size = window_size()?;
        boid_settings.update_window(size.columns as usize, size.rows as usize * 2);
        while poll(Duration::from_millis(0))? {
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Esc => break 'simulation,
                    KeyCode::Char('q') => break 'simulation,
                    _ => (),
                },
                Event::Mouse(event) => {
                    boid_settings.set_mouse_position(event.column as f64, event.row as f64 * 2.0);
                }
                Event::FocusGained => {
                    boid_settings.set_mouse_force(MOUSE_FORCE, MOUSE_RANGE);
                }
                Event::FocusLost => {
                    boid_settings.set_mouse_force(0.0, 0.0);
                }
                _ => (),
            }
        }
        queue!(stdout, Clear(ClearType::All))?;
        update_boids(&mut boid_data, &boid_settings, 0.1);

        draw_boids(&mut stdout, &boid_data.boids, &size, &boid_settings)?;
        queue!(stdout, MoveTo(0, 0))?;
        let current_frame_time = now.elapsed();
        stdout.flush()?;
        if current_frame_time.as_millis() < FRAME_TIME.as_millis() {
            sleep(FRAME_TIME.abs_diff(current_frame_time));
        }
    }
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
    run()
}
