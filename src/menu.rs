//! Functionality and rendering of a menu to control the boid simulation.
//!
//! # Menu
//! WIP

use std::io::{Result, Stdout, stdout};

use crossterm::{
    cursor::MoveTo,
    event::{Event, KeyCode, KeyEvent},
    queue,
    style::{
        Color::{DarkGrey, White},
        Colors, Print, SetColors,
    },
};

/// The different possible items appearing in the menu,
/// along with values, settings and generic identifiers, meant for
/// the calling functions to identify which settings were changed.
pub enum MenuItem<'a, T> {
    /// An integer number input with specified `min` and `max` constraints.
    #[allow(dead_code)]
    IntSlider {
        id: T,
        current: i32,
        min: i32,
        max: i32,
    },
    /// A decimal number input with specified `min`, `max` and
    /// `step_size` as constraints when changing the value.
    #[allow(dead_code)]
    FloatSlider {
        id: T,
        current: f32,
        min: f32,
        max: f32,
        step_size: f32,
    },
    /// A toggle between true or false.
    #[allow(dead_code)]
    Toggle { id: T, current: bool },
    /// Allows for the choice of a string value from `options`, can be used when
    /// a [`MenuItem::Toggle`] is too restrictive.
    #[allow(dead_code)]
    Choice {
        id: T,
        current: usize,
        options: Vec<&'a str>,
    },
}

impl<'a, T> MenuItem<'a, T> {
    #[allow(dead_code)]
    fn alter(&mut self, factor: i32) {
        match self {
            MenuItem::IntSlider {
                current, max, min, ..
            } => {
                *current = (*max).min(*current + factor).max(*min);
            }
            MenuItem::FloatSlider {
                current,
                max,
                min,
                step_size,
                ..
            } => {
                *current = (*max)
                    .min(*current + (factor as f32) * *step_size)
                    .max(*min);
            }
            MenuItem::Toggle { current, .. } => *current = !*current,
            MenuItem::Choice {
                current, options, ..
            } => *current = (*current as i32 + factor).rem_euclid(options.len() as i32) as usize,
        }
    }
}

/// A collection of [`MenuItem`]s, together forming a menu.
pub struct Menu<'a, T> {
    /// The menu items forming the menu. The order of occurence is the same
    /// between this vector and the rendered elements.
    items: Vec<MenuItem<'a, T>>,
    /// The names of the respective menu items.
    names: Vec<&'a str>,
    /// The index of the currently selected element in the menu.
    current: usize,
}

impl<'a, T> Menu<'a, T> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Menu {
            items: Vec::new(),
            names: Vec::new(),
            current: 0,
        }
    }

    /// Add a new `menu_item` to the end of the menu.
    #[allow(dead_code)]
    pub fn add_menu_item(&mut self, menu_item: MenuItem<'a, T>, name: &'a str) -> &mut Menu<'a, T> {
        self.items.push(menu_item);
        self.names.push(name);
        self
    }
}

fn handle_key_event<'a, T>(menu: &mut Menu<'a, T>, key_event: &KeyEvent) -> bool {
    match key_event.code {
        KeyCode::Left => {
            menu.items[menu.current].alter(-1);
            true
        }
        KeyCode::Down => {
            menu.items[menu.current].alter(-10);
            true
        }
        KeyCode::Right => {
            menu.items[menu.current].alter(1);
            true
        }
        KeyCode::Up => {
            menu.items[menu.current].alter(10);
            true
        }
        KeyCode::Tab => {
            menu.current = (menu.current + 1) % menu.items.len();
            false
        }
        KeyCode::BackTab => {
            menu.current = (menu.current as i32 - 1).rem_euclid(menu.items.len() as i32) as usize;
            false
        }
        _ => false,
    }
}

/// TODO.
pub fn handle_input<'a: 'b, 'b, T>(
    menu: &'b mut Menu<'a, T>,
    event: &Event,
) -> Option<&'b MenuItem<'a, T>> {
    match event {
        Event::Key(key_event) => {
            if handle_key_event(menu, key_event) {
                Some(&menu.items[menu.current])
            } else {
                None
            }
        }
        _ => None,
    }
}

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn draw_item<'a, T>(item: &MenuItem<'a, T>, stdout: &mut Stdout) -> Result<()> {
    match item {
        MenuItem::IntSlider { current, .. } => {
            queue!(stdout, Print("< "), Print(current), Print(" >"))?
        }
        MenuItem::FloatSlider { current, .. } => queue!(
            stdout,
            Print("< "),
            Print(format!("{:.2}", current)),
            Print(" >")
        )?,
        MenuItem::Toggle { current, .. } => {
            if *current {
                queue!(stdout, Print("[x]"))?;
            } else {
                queue!(stdout, Print("[ ]"))?;
            }
        }
        MenuItem::Choice {
            current, options, ..
        } => {
            queue!(stdout, Print("< "), Print(options[*current]), Print(" >"))?;
        }
    }
    Ok(())
}

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn draw_menu<'a, T>(menu: &Menu<'a, T>) -> Result<()> {
    let mut stdout = stdout();
    let name_color = Colors::new(White, DarkGrey);
    let chosen_color = Colors::new(DarkGrey, White);
    queue!(stdout, SetColors(name_color))?;
    for i in 0..menu.names.len() {
        if i == menu.current {
            queue!(
                stdout,
                MoveTo(0, i as u16),
                SetColors(chosen_color),
                Print(menu.names[i]),
                SetColors(name_color)
            )?;
            draw_item(&menu.items[i], &mut stdout)?;
        } else {
            queue!(stdout, MoveTo(0, i as u16,), Print(menu.names[i]))?;
        }
    }
    Ok(())
}
