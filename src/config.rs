// Copyright © 2021-22 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::fixed::{
    APPNAME, HISTORY_SIZE, SCALE_MAX, SCALE_MIN, WINDOW_HEIGHT_MIN,
    WINDOW_WIDTH_MIN,
};
use crate::util;

#[derive(Clone, Debug)]
pub struct Config {
    pub window_x: i32,
    pub window_y: i32,
    pub window_height: i32,
    pub window_width: i32,
    pub window_scale: f32,
    pub filename: std::path::PathBuf,
    pub searches: [String; HISTORY_SIZE],
    pub history: [char; HISTORY_SIZE],
}

impl Config {
    pub fn new() -> Self {
        let mut config = Config {
            filename: get_config_filename(),
            ..Default::default()
        };
        if let Ok(ini) = ini::Ini::load_from_file(&config.filename) {
            if let Some(properties) = ini.section(Some(WINDOW_SECTION)) {
                read_window_properties(properties, &mut config);
            }
            if let Some(properties) = ini.section(Some(GENERAL_SECTION)) {
                read_general_properties(properties, &mut config);
            }
        }
        config
    }

    pub fn save(&self, x: i32, y: i32, width: i32, height: i32) {
        if self.filename.to_string_lossy() == "" {
            self.warning("failed to save configuration: no filename");
        } else {
            let mut ini = ini::Ini::new();
            ini.with_section(Some(WINDOW_SECTION))
                .set(X_KEY, x.to_string())
                .set(Y_KEY, y.to_string())
                .set(WIDTH_KEY, width.to_string())
                .set(HEIGHT_KEY, height.to_string())
                .set(SCALE_KEY, fltk::app::screen_scale(0).to_string());
            ini.with_section(Some(GENERAL_SECTION))
                .set(HISTORY_KEY, self.history_str());
            self.save_searches(&mut ini);
            match ini.write_to_file(&self.filename) {
                Ok(_) => {}
                Err(err) => self.warning(&format!(
                    "failed to save configuration: {}",
                    err
                )),
            }
        }
    }

    fn history_str(&self) -> String {
        let mut history = String::new();
        for i in 0..HISTORY_SIZE {
            history.push(self.history[i]);
        }
        history
    }

    fn save_searches(&self, ini: &mut ini::Ini) {
        for i in 0..HISTORY_SIZE {
            let key = format!("{}{}", SEARCH_KEY, i + 1);
            ini.with_section(Some(GENERAL_SECTION))
                .set(key, self.searches[i].clone());
        }
    }

    fn warning(&self, message: &str) {
        fltk::dialog::message_title(&format!("Warning — {}", APPNAME));
        fltk::dialog::message(util::x() - 200, util::y() - 100, message);
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_x: -1,
            window_y: -1,
            window_height: WINDOW_HEIGHT_MIN,
            window_width: WINDOW_WIDTH_MIN,
            window_scale: 1.0,
            filename: std::path::PathBuf::new(),
            searches: [
                "arrow".to_string(),
                "asterisk".to_string(),
                "block".to_string(),
                "box".to_string(),
                "euro".to_string(),
                "fraction".to_string(),
                "geometric".to_string(),
                "greek symbol".to_string(),
                "technical".to_string(),
            ],
            history: ['•', '…', '—', '€', '£', '←', '→', '↑', '↓'],
        }
    }
}

fn get_config_filename() -> std::path::PathBuf {
    let mut dir = dirs::config_dir();
    let mut dot = "";
    if dir.is_none() {
        if std::env::consts::FAMILY == "unix" {
            dot = ".";
        }
        dir = dirs::home_dir();
    }
    if let Some(dir) = dir {
        dir.join(format!("{}{}.ini", dot, APPNAME.to_lowercase()))
    } else {
        std::path::PathBuf::new()
    }
}

fn read_window_properties(
    properties: &ini::Properties,
    config: &mut Config,
) {
    let max_x = (fltk::app::screen_size().0 - 100.0) as i32;
    let max_y = (fltk::app::screen_size().1 - 100.0) as i32;
    if let Some(value) = properties.get(X_KEY) {
        config.window_x = util::get_num(value, 0, max_x, config.window_x)
    }
    if let Some(value) = properties.get(Y_KEY) {
        config.window_y = util::get_num(value, 0, max_y, config.window_y)
    }
    if let Some(value) = properties.get(WIDTH_KEY) {
        config.window_width =
            util::get_num(value, 200, max_x, config.window_width)
    }
    if let Some(value) = properties.get(HEIGHT_KEY) {
        config.window_height =
            util::get_num(value, 240, max_y, config.window_height)
    }
    if let Some(value) = properties.get(SCALE_KEY) {
        config.window_scale =
            util::get_num(value, SCALE_MIN, SCALE_MAX, config.window_scale);
        if !util::isone32(config.window_scale) {
            fltk::app::set_screen_scale(0, config.window_scale);
        }
    }
}

fn read_general_properties(
    properties: &ini::Properties,
    config: &mut Config,
) {
    if let Some(value) = properties.get(HISTORY_KEY) {
        for (i, c) in value.chars().enumerate() {
            config.history[i] = c;
        }
    }
    for i in 0..HISTORY_SIZE {
        let key = format!("{}{}", SEARCH_KEY, i + 1);
        if let Some(value) = properties.get(&key) {
            config.searches[i] = value.to_string();
        }
    }
}

static WINDOW_SECTION: &str = "Window";
static X_KEY: &str = "x";
static Y_KEY: &str = "y";
static WIDTH_KEY: &str = "width";
static HEIGHT_KEY: &str = "height";
static SCALE_KEY: &str = "scale";
static GENERAL_SECTION: &str = "General";
static HISTORY_KEY: &str = "history";
static SEARCH_KEY: &str = "search";
