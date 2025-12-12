//! Functionality and rendering of a menu to control the boid simulation.
//!
//! # Menu
//! WIP

use crossterm::event::{Event, KeyCode, KeyEvent};

/// The different possible items appearing in the menu,
/// along with values, settings and generic identifiers, meant for
/// the calling functions to identify which settings were changed.
pub enum MenuItem<'a, T> {
    /// An integer number input with specified `min` and `max` constraints.
    IntSlider {
        id: T,
        current: i32,
        min: i32,
        max: i32,
    },
    /// A decimal number input with specified `min`, `max` and
    /// `step_size` as constraints when changing the value.
    FloatSlider {
        id: T,
        current: f32,
        min: f32,
        max: f32,
        step_size: f32,
    },
    /// A toggle between true or false.
    Toggle { id: T, current: bool },
    /// Allows for the choice of a string value from `options`, can be used when
    /// a [`MenuItem::Toggle`] is too restrictive.
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
            MenuItem::IntSlider { current, max, .. } => {
                *current = (*max).min(*current + factor);
            }
            MenuItem::FloatSlider {
                current,
                max,
                step_size,
                ..
            } => {
                *current = (*max).min(*current + (factor as f32) * *step_size);
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
    /// The index of the currently selected element in the menu.
    current: usize,
}

impl<'a, T> Menu<'a, T> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Menu {
            items: Vec::new(),
            current: 0,
        }
    }

    /// Add a new `menu_item` to the end of the menu.
    #[allow(dead_code)]
    pub fn add_menu_item(&mut self, menu_item: MenuItem<'a, T>) {
        self.items.push(menu_item);
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
            menu.current = (menu.current - 1).rem_euclid(menu.items.len());
            false
        }
        _ => false,
    }
}

pub fn handle_input<'a, T>(
    menu: &'a mut Menu<'a, T>,
    event: &Event,
) -> Option<&'a MenuItem<'a, T>> {
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

pub fn draw_menu<'a, T>(menu: &Menu<'a, T>) {}
