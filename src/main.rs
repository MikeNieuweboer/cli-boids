// MVP
// TODO: Custom input.

// Extra's
// TODO: Path showing
// TODO: Path drawing
// TODO: Arrows for boids
// TODO: 3D boids?

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{
        DisableFocusChange, DisableMouseCapture, EnableFocusChange, EnableMouseCapture, Event,
        KeyCode, MouseButton, MouseEventKind, poll, read,
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
pub mod grid;
pub mod vector2;

use crate::boids::{Boid, BoidSettings, BorderSettings, populate, resize_grid, update_boids};

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

fn pos_to_braille(x_norm: f32, y_norm: f32) -> u8 {
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
    boids: &Vec<grid::ValueNode<Boid>>,
    window_size: &WindowSize,
    boid_settings: &BoidSettings,
) -> Result<()> {
    let rows = window_size.rows;
    let columns = window_size.columns;

    let mut braille_grid = vec![0u8; (rows as usize) * (columns as usize)];
    let width_ratio: f32 = (columns as f32) / (boid_settings.width as f32);
    let height_ratio: f32 = (rows as f32) / (boid_settings.height as f32);

    for boid in boids {
        let position = boid.val.position;
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

    let mut size = window_size()?;
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
    let mut boid_data = populate(COUNT, 2, &boid_settings);

    let mut last_duration: f32 = 0.02;
    let mut pause = false;
    'simulation: loop {
        let now = Instant::now();
        while poll(Duration::from_millis(0))? {
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Esc => break 'simulation,
                    KeyCode::Char(' ') => {
                        if pause {
                            pause = false;
                            execute!(stdout, Hide, EnableMouseCapture)?;
                        } else {
                            pause = true;
                            execute!(stdout, Show, DisableMouseCapture)?;
                        }
                    }
                    KeyCode::Char('q') => break 'simulation,
                    _ => (),
                },
                Event::Mouse(event) => {
                    match event.kind {
                        MouseEventKind::Down(MouseButton::Left) => {
                            boid_settings.set_mouse_force(MOUSE_FORCE_DOWN, MOUSE_RANGE_DOWN);
                        }
                        MouseEventKind::Up(MouseButton::Left) => {
                            boid_settings.set_mouse_force(MOUSE_FORCE, MOUSE_RANGE);
                        }
                        _ => (),
                    }
                    boid_settings.set_mouse_position(
                        event.column as f32 + 0.5,
                        event.row as f32 * 2.0 + 1.0,
                    );
                }
                Event::FocusGained => {
                    boid_settings.set_mouse_force(MOUSE_FORCE, MOUSE_RANGE);
                }
                Event::FocusLost => {
                    boid_settings.set_mouse_force(0.0, 0.0);
                }
                Event::Resize(columns, rows) => {
                    boid_settings.update_window(columns as usize, rows as usize * 2);
                    size.rows = rows;
                    size.columns = columns;
                    boid_data = resize_grid(boid_data, &boid_settings);
                }
                _ => (),
            }
        }
        if pause {
            continue;
        }
        queue!(stdout, Clear(ClearType::All))?;
        const FACTOR: f32 = 10.0;
        update_boids(&mut boid_data, &boid_settings, last_duration * FACTOR);

        draw_boids(&mut stdout, &boid_data.values, &size, &boid_settings)?;
        queue!(stdout, MoveTo(0, 0))?;
        let current_frame_time = now.elapsed();
        stdout.flush()?;
        if current_frame_time.as_millis() < FRAME_TIME.as_millis() {
            sleep(FRAME_TIME.abs_diff(current_frame_time));
            last_duration = FRAME_TIME.as_millis() as f32 / 1000.0;
        } else {
            last_duration = current_frame_time.as_millis() as f32 / 1000.0;
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
