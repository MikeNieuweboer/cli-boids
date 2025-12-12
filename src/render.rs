//! Functions for rendering of boids.
//!
//! # Rendering
//!
//! Supplies functionality to discretize and render the boids in the terminal
//! using utf16 braille characters.

use std::io::{Result, Stdout};

use crossterm::{cursor::MoveTo, queue, style::Print, terminal::WindowSize};

use crate::boids::{Boid, settings::BoidSettings};

/// Prints the boids in the terminal using braille characters as pixels.
///
/// # Errors
///
/// This function will return an error if it fails to queue its drawing operation.
pub fn draw_boids<'a>(
    stdout: &mut Stdout,
    boids: impl Iterator<Item = &'a Boid>,
    window_size: &WindowSize,
    boid_settings: &BoidSettings,
) -> Result<()> {
    let rows = window_size.rows;
    let columns = window_size.columns;

    // Temporary grid for or'ing braille codes
    let mut braille_grid = vec![0u8; (rows as usize) * (columns as usize)];

    let width_ratio: f32 = (columns as f32) / (boid_settings.width as f32);
    let height_ratio: f32 = (rows as f32) / (boid_settings.height as f32);

    for boid in boids {
        // Determine the boid's character position
        let position = boid.position;
        let x = position.x * width_ratio;
        let c = x.floor();
        if c as u16 >= columns || c < 0.0 {
            continue;
        }
        let y = position.y * height_ratio;
        let r = y.floor();
        if r as u16 >= rows || r < 0.0 {
            continue;
        }

        // Braille based on position within character
        let braille = pos_to_braille(x - c, y - r);

        // As braille is like binary, the boids can be or'ed to merge characters.
        braille_grid[(c as usize) + (r as usize) * (columns as usize)] |= braille;
    }

    // Print boids based on utf16 braile codes.
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

/// Returns the least significant part of the brailles utf16 based on where in
/// the character the point is present.
///
/// # Examples
/// ```
/// let braille = pos_to_braille(0.4, 0.1);
///
/// // Prints the ⠁ character.
/// println!(String::from_utf16(&[0x2800 | braille]));
///
/// assert_eq!(braille, 0b00000001);
/// ```
fn pos_to_braille(x_norm: f32, y_norm: f32) -> u8 {
    let mut braille: u8 = 1;
    if y_norm <= 0.75 {
        // Interpret braille as column order bits for first 6 dots.
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
    } else {
        // Interpret braille as row order bits for last 2 dots.
        if x_norm <= 0.5 {
            braille <<= 6;
        } else {
            braille <<= 7;
        }
    }
    braille
}
