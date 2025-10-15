use crossterm::{
    cursor::{Hide, MoveTo},
    event::{Event, KeyCode, KeyEvent, poll, read},
    execute, queue, style,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode, window_size,
    },
};
use std::{
    io::{Result, Stdout, Write, stdout},
    time::Duration,
};

const MIN_SPEED: u32 = 4;
const MAX_SPEED: u32 = 6;
const COUNT: usize = 1;

#[derive(Clone, Copy, Debug)]
struct Vector2 {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Boid {
    position: Vector2,
    velocity: Vector2,
}

impl Vector2 {
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn populate(count: usize, width: usize, height: usize) -> Vec<Boid> {
    let mut boids: Vec<Boid> = Vec::with_capacity(count);
    let position = Vector2 {
        x: (width as f64) / 2f64,
        y: (height as f64) / 2f64,
    };
    let velocity = Vector2 { x: 0f64, y: 0f64 };
    for _ in 0..count {
        boids.push(Boid { position, velocity });
    }
    boids
}

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

fn run() -> Result<()> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, Clear(ClearType::All), Hide)?;

    let size = window_size()?;
    let boids = populate(COUNT, size.columns as usize, size.rows as usize);
    loop {
        if poll(Duration::from_millis(500))?
            && let Event::Key(event) = read()?
        {
            match event.code {
                KeyCode::Esc => break,
                KeyCode::Char('q') => break,
                _ => (),
            }
        }
        draw_boid(&mut stdout, &boids[0]);
        stdout.flush()?;
    }
    execute!(stdout, LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

fn main() -> Result<()> {
    run()
}
