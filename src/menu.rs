//! Functionality and rendering of a menu to control the boid simulation.
//!
//! # Menu
//! Allows for the creation and rendering of a menu by creating a [`Menu<T>`]
//! struct ! containing a [`MenuItem<T>`] for each option. Here the dynamic type
//! `T` should be an unique identifier for each item, allowing for a mapping
//! between items in the menu and actions in the code using it.
// TODO: Give some examples on how to render and how to handle input events

use std::io::{Result, Stdout, stdout};

use crossterm::{
    cursor::MoveTo,
    event::{Event, KeyCode, KeyEvent},
    queue,
    style::{
        Color::{Black, White},
        Colors, Print, SetColors,
    },
};

/// The different possible items appearing in the menu,
/// along with values, settings and generic identifiers, meant for
/// the calling functions to identify which settings were changed.
pub enum MenuItem<T> {
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
        options: Vec<&'static str>,
    },
}

impl<T> MenuItem<T> {
    /// Alters the value in `self` according to the given factor:
    /// - Increases or decreases a slider by the given `factor`.
    /// - Toggles a toggle.
    /// - Increases or decreases the index of the selected choice according to
    ///   the sign of the `factor`.
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
            } => {
                *current =
                    (*current as i32 + factor.signum()).rem_euclid(options.len() as i32) as usize
            }
        }
    }
}

/// A collection of [`MenuItem`]s, together forming a menu.
pub struct Menu<T> {
    /// The menu items forming the menu. The order of occurence is the same
    /// between this vector and the rendered elements.
    items: Vec<MenuItem<T>>,
    /// The names of the respective menu items.
    names: Vec<&'static str>,
    /// The index of the currently selected element in the menu.
    current: usize,
    width: u16,
}

impl<T> Menu<T> {
    /// Creates a new [`Menu<T>`].
    #[allow(dead_code)]
    pub fn new() -> Self {
        Menu {
            items: Vec::new(),
            names: Vec::new(),
            width: 0,
            current: 0,
        }
    }

    /// Add a new `menu_item` to the end of the menu.
    #[allow(dead_code)]
    pub fn add_menu_item(&mut self, menu_item: MenuItem<T>, name: &'static str) -> &mut Menu<T> {
        self.items.push(menu_item);
        self.names.push(name);
        self.width = self.width.max(name.chars().count() as u16);
        self
    }
}

/// Handles the input `key_event` in case of a [`KeyEvent`], allowing for the
/// changing of values using the directional input, or traversal of menu using tab and backtab.
///
/// # Return
/// Returns `true` if a menu item's value is changed, `false` otherwise.
fn handle_key_event<T>(menu: &mut Menu<T>, key_event: &KeyEvent) -> bool {
    const SMALL_STEP: i32 = 1;
    const LARGE_STEP: i32 = 10;
    match key_event.code {
        KeyCode::Left => {
            menu.items[menu.current].alter(-SMALL_STEP);
            true
        }
        KeyCode::Down => {
            menu.items[menu.current].alter(-LARGE_STEP);
            true
        }
        KeyCode::Right => {
            menu.items[menu.current].alter(SMALL_STEP);
            true
        }
        KeyCode::Up => {
            menu.items[menu.current].alter(LARGE_STEP);
            true
        }
        // Move down in menu
        KeyCode::Tab => {
            menu.current = (menu.current + 1) % menu.items.len();
            false
        }
        // Move up in menu
        KeyCode::BackTab => {
            menu.current = (menu.current as i32 - 1).rem_euclid(menu.items.len() as i32) as usize;
            false
        }
        _ => false,
    }
}

/// Handles the input event and returns either some changed [`MenuItem<T>`], or None.
pub fn handle_input<'b, T>(menu: &'b mut Menu<T>, event: &Event) -> Option<&'b MenuItem<T>> {
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

/// Draws the interactable part of a menu item, which is either a:
/// - Slider (< 10.0 >)
/// - Toggle ([x])
/// - Choice (< option1 >)
///
/// # Errors
///
/// This function will return an error if it fails to interact with the terminal.
pub fn draw_item<T>(item: &MenuItem<T>, stdout: &mut Stdout) -> Result<()> {
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

/// Draws the menu and the currently selected option in the top left corner of
/// the terminal. The menu is as wide as the largest string contained.
///
/// # Note
/// Changes the color used for rendering to the terminal.
///
/// # Errors
///
/// This function will return an error if it fails to interact with the terminal.
pub fn draw_menu<T>(menu: &Menu<T>) -> Result<()> {
    let mut stdout = stdout();
    let name_color = Colors::new(Black, White);
    let chosen_color = Colors::new(White, Black);
    queue!(stdout, SetColors(name_color))?;

    for i in 0..menu.names.len() {
        // If selected
        if i == menu.current {
            queue!(
                stdout,
                MoveTo(0, i as u16),
                SetColors(chosen_color),
                Print(format!("{:<1$}", menu.names[i], menu.width as usize)),
                SetColors(name_color)
            )?;
            draw_item(&menu.items[i], &mut stdout)?;
        } else {
            // If not selected
            queue!(
                stdout,
                MoveTo(0, i as u16,),
                Print(format!("{:<1$}", menu.names[i], menu.width as usize))
            )?;
        }
    }
    Ok(())
}
