// TODO: Custom input

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{Event, KeyCode, KeyEvent, poll, read},
    execute, queue,
    style::{self, Print},
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, WindowSize, disable_raw_mode,
        enable_raw_mode, window_size,
    },
};
use std::{
    io::{Result, Stdout, Write, stdout},
    time::Duration,
};

pub mod boids;
pub mod vector2;

use crate::boids::{Boid, BoidSettings, populate, update_boids};

const MIN_SPEED: f64 = 2f64;
const MAX_SPEED: f64 = 4f64;
const SEPERATION_DIST: f64 = 2f64;
const COHESION_DIST: f64 = 20f64;
const COUNT: usize = 1000;
const MARGIN: f64 = 20.0;

fn draw_boid(stdout: &mut Stdout, boid: &Boid) {
    let position = boid.position;
    let row: u16 = position.x.round() as u16;
    let column: u16 = position.y.round() as u16;
    match queue!(stdout, MoveTo(row, column), style::Print(".".to_string())) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Couldn't draw boid");
        }
    }
}

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
    execute!(stdout, EnterAlternateScreen, Clear(ClearType::All), Hide)?;

    let size = window_size()?;
    let height = (size.rows * 2u16) as usize;
    let width = size.columns as usize;
    let mut boid_settings = BoidSettings::new(
        SEPERATION_DIST,
        COHESION_DIST,
        width,
        height,
        MIN_SPEED,
        MAX_SPEED,
        MARGIN,
    );
    let mut boids = populate(COUNT, &boid_settings);
    loop {
        // TODO: Set polling time to take processing time account.
        let size = window_size()?;
        boid_settings.width = size.columns as usize;
        boid_settings.height = size.rows as usize * 2;
        if poll(Duration::from_millis(10))?
            && let Event::Key(event) = read()?
        {
            match event.code {
                KeyCode::Esc => break,
                KeyCode::Char('q') => break,
                _ => (),
            }
        }
        queue!(stdout, Clear(ClearType::All))?;
        update_boids(&mut boids, &boid_settings, 0.1);

        draw_boids(&mut stdout, &boids, &size, &boid_settings)?;
        queue!(stdout, MoveTo(0, 0))?;
        stdout.flush()?;
    }
    execute!(stdout, LeaveAlternateScreen, Show)?;
    disable_raw_mode()?;

    Ok(())
}

fn main() -> Result<()> {
    run()
}
