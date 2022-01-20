// Copyright © 2021-22 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::fixed::{Action, AUTO_MENU_SIZE, MENU_CHARS};
use fltk::prelude::*;
use std::{cmp, fmt, str};

pub fn x() -> i32 {
    (fltk::app::screen_size().0 / 2.0) as i32
}

pub fn y() -> i32 {
    (fltk::app::screen_size().1 / 2.0) as i32
}

pub fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

// Returns a number converted from the given str or the default
pub fn get_num<T>(s: &str, minimum: T, maximum: T, default: T) -> T
where
    T: num::Num + cmp::PartialOrd + Copy + str::FromStr,
{
    match s.parse() {
        Ok(n) if minimum <= n && n <= maximum => n,
        _ => default,
    }
}

pub fn isclose32(a: f32, b: f32) -> bool {
    (a..=(a + f32::EPSILON)).contains(&b)
}

pub fn isone32(n: f32) -> bool {
    (1.0..=(1.0 + f32::EPSILON)).contains(&n)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub const INVALID: i32 = -1;

    #[allow(dead_code)]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[allow(dead_code)]
    pub fn is_valid(&self) -> bool {
        self.x != Pos::INVALID && self.y != Pos::INVALID
    }
}

impl Default for Pos {
    fn default() -> Self {
        Self { x: Pos::INVALID, y: Pos::INVALID }
    }
}

impl std::fmt::Display for Pos {
    fn fmt(&self, out: &mut std::fmt::Formatter) -> fmt::Result {
        write!(out, "({},{})", self.x, self.y)
    }
}

pub fn string_for_codepoint(cp: u32) -> String {
    if cp <= 0xFFFF {
        format!("  {cp:04X}")
    } else {
        format!("{cp:>6X}")
    }
}

pub fn add_to_history(c: char) -> bool {
    {
        let config = CONFIG.get().read().unwrap();
        if config.history.contains(&c) {
            return false;
        }
    }
    let mut config = CONFIG.get().write().unwrap();
    config.history.push_front(c);
    config.history.truncate(AUTO_MENU_SIZE);
    true
}

pub fn add_to_searches(s: &str) -> bool {
    let mut config = CONFIG.get().write().unwrap();
    if let Some(front) = config.searches.front() {
        if front == s {
            return false; // The new string is already the first one
        }
    }
    // If the same as an existing one, move the existing one to the front
    let mut found = false;
    let mut i = 0;
    for t in config.searches.iter() {
        if t == s {
            found = true;
            break;
        }
        i += 1;
    }
    let s = s.to_string();
    if found {
        config.searches.remove(i);
        config.searches.push_front(s);
        return true;
    }
    // If the first one is almost the same as the new one replace with new
    if let Some(front) = config.searches.front_mut() {
        if s.starts_with(front.as_str())
            || levenshtein::levenshtein(&s, front.as_str()) < 2
        {
            *front = s;
            return true;
        }
    }
    config.searches.push_front(s);
    config.searches.truncate(AUTO_MENU_SIZE);
    true
}

pub fn populate_find_combo(
    find_combo: &mut fltk::misc::InputChoice,
    sender: fltk::app::Sender<Action>,
) {
    find_combo.clear();
    let config = CONFIG.get().read().unwrap();
    let size = config.searches_size;
    let base = if (10..=26).contains(&size) { 9 } else { 0 };
    for (i, s) in config.searches.iter().enumerate() {
        if i == size {
            break;
        }
        find_combo.menu_button().add_emit(
            &format!("&{} {s}", MENU_CHARS[base + i]),
            fltk::enums::Shortcut::None,
            fltk::menu::MenuFlag::Normal,
            sender,
            Action::SearchFor(i as i32),
        );
    }
}
