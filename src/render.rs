use std::io::{Result, Stdout};

use crossterm::{cursor::MoveTo, queue, style::Print, terminal::WindowSize};

use crate::{
    boids::{Boid, BoidSettings},
    grid::ValueNode,
};

// TODO: Make the values nodes an iterable?

pub fn draw_boids(
    stdout: &mut Stdout,
    boids: &Vec<ValueNode<Boid>>,
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
