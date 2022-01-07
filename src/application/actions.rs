// Copyright © 2021-22 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::fixed::{about_html, CHARDATA, HELP_HTML};
use crate::html_form;
use crate::options_form;
use crate::Application;
use flate2::read::GzDecoder;
use fltk::prelude::*;
use std::collections::HashSet;
use std::io::prelude::*;
use std::iter::Iterator;

type WordSet = HashSet<String>;

impl Application {
    pub(crate) fn on_search(&mut self) {
        let words = self.get_search_words();
        if words.is_empty() {
            return; // nothing to search for
        }
        // TODO clear the table
        let match_all = self.all_radio.is_toggled();
        let (cp1, cp2) = self.get_code_points(&words);
        self.maybe_populate_chardata();
        let mut found = false;
        if let Some(chardata) = &self.chardata {
            for line in chardata.lines() {
                let (cp, desc, keywords) = self.get_unicode_data(&line);
                let matched = if cp != 0 && (cp == cp1 || cp == cp2) {
                    true
                } else if match_all {
                    words.intersection(&keywords).count() == words.len()
                } else {
                    words.intersection(&keywords).count() > 0
                };
                if matched {
                    found = true;
                    if let Some(c) = char::from_u32(cp) {
                        // TODO add row to the table
                        println!(
                            "match cp={} char={} desc={}",
                            cp, c, desc
                        );
                    }
                }
            }
        }
        if found {
            self.update_searches();
        }
    }

    fn maybe_populate_chardata(&mut self) {
        if self.chardata.is_none() {
            let mut gz = GzDecoder::new(CHARDATA);
            let mut text = String::new();
            gz.read_to_string(&mut text)
                .expect("failed to read internal Unicode character data");
            self.chardata = Some(text);
        }
    }

    fn get_search_words(&self) -> WordSet {
        match self.find_combo.value() {
            None => WordSet::new(),
            Some(words) => words
                .to_uppercase()
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect(),
        }
    }

    fn get_code_points(&self, words: &WordSet) -> (u32, u32) {
        let mut cp1 = 0;
        let mut cp2 = 0;
        for word in words {
            match word.parse::<u32>() {
                Ok(cp) => {
                    cp1 = cp;
                }
                Err(_) => (),
            }
            match u32::from_str_radix(&word, 16) {
                Ok(cp) => {
                    cp2 = cp;
                }
                Err(_) => (),
            }
            if cp1 != 0 && cp2 != 0 {
                break;
            }
        }
        (cp1, cp2)
    }

    fn get_unicode_data(&self, line: &str) -> (u32, String, WordSet) {
        let cols = line.split('\t').collect::<Vec<&str>>();
        let cp = cols[0].parse::<u32>().unwrap_or(0);
        let keywords = cols[2]
            .split('\x0C')
            .map(|s| s.to_owned())
            .collect::<WordSet>();
        (cp, cols[1].to_string(), keywords)
    }

    fn update_searches(&mut self) {
        // TODO add the find_combo's value() to config searches
        // 1. first ripple down HISTORY_SIZE -2 → HISTORY_SIZE -1,
        // HISTORY_SIZE -3 → HISTORY_SIZE -2 ...
        // 2. then add the value as the new first entry
        // 3. update find_combo's list
        // 4. refactor into
    }

    pub(crate) fn on_copy(&mut self) {
        println!("on_copy"); // TODO copy copy_input to clipboard
    }

    pub(crate) fn on_add_char(&mut self, c: char) {
        println!("on_add_char({})", c); // TODO // add to copy_input
    }

    pub(crate) fn on_add_from_table(&mut self) {
        println!("on_add_from_table"); // TODO // add to copy_input
    }

    pub(crate) fn on_options(&mut self) {
        options_form::Form::default();
    }

    pub(crate) fn on_about(&mut self) {
        html_form::Form::new("About", &about_html(), true, 480, 280, false);
    }

    pub(crate) fn on_help(&mut self) {
        if let Some(help_form) = &mut self.help_form {
            help_form.show();
        } else {
            self.help_form = Some(html_form::Form::new(
                "Help", HELP_HTML, false, 380, 420, true,
            ));
        }
    }

    pub(crate) fn on_quit(&mut self) {
        let config = CONFIG.get().read().unwrap();
        // TODO save history & searches not here but AS WE GO!
        config.save(
            self.main_window.x(),
            self.main_window.y(),
            self.main_window.width(),
            self.main_window.height(),
        );
        self.app.quit();
    }
}
