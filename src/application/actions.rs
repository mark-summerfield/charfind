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
use thousands::Separable;

type WordSet = HashSet<String>;

impl Application {
    pub(crate) fn on_search(&mut self) {
        let (words, ignores) = self.get_search_words();
        if words.is_empty() {
            return; // nothing to search for
        }
        self.browser.clear();
        let (cp1, cp2) = self.get_code_points(&words);
        self.maybe_populate_chardata();
        let mut n = 1;
        if let Some(chardata) = &self.chardata {
            for line in chardata.lines() {
                let (cp, desc, keywords) = self.get_unicode_data(line);
                if self.is_match(cp, cp1, cp2, &words, &keywords)
                    && ignores.intersection(&keywords).count() == 0
                {
                    if let Some(c) = char::from_u32(cp) {
                        n += 1;
                        let bg = if n % 2 == 0 { "@B247" } else { "" };
                        let cp = string_for_codepoint(cp);
                        self.browser.insert(
                            n,
                            &format!("{}@t{}\t{}\t{}", bg, c, cp, desc),
                        );
                    }
                }
            }
        }
        self.add_header_line(n - 1);
    }

    fn is_match(
        &self,
        cp: u32,
        cp1: u32,
        cp2: u32,
        words: &WordSet,
        keywords: &WordSet,
    ) -> bool {
        if cp != 0 && (cp == cp1 || cp == cp2) {
            true
        } else if self.all_radio.is_toggled() {
            words.intersection(keywords).count() == words.len()
        } else {
            words.intersection(keywords).count() > 0
        }
    }

    fn add_header_line(&mut self, n: i32) {
        if n > 0 {
            let s = if n > 1 { "es" } else { "" };
            self.browser.insert(
                1,
                &format!(
                    "@C7@B136@t@bChar\tU+HHHH\tDescription ({} match{})",
                    n.separate_with_commas(),
                    s
                ),
            );
            self.update_searches();
        } else {
            self.browser.insert(1, "@B3@C1No matches found");
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

    fn get_search_words(&self) -> (WordSet, WordSet) {
        if let Some(line) = self.find_combo.value() {
            let mut words = WordSet::new();
            let mut ignores = WordSet::new();
            for word in line.split_whitespace() {
                if let Some(word) = word.strip_prefix('-') {
                    ignores.insert(word.to_uppercase());
                } else {
                    words.insert(word.to_uppercase());
                }
            }
            (words, ignores)
        } else {
            (WordSet::new(), WordSet::new())
        }
    }

    fn get_code_points(&self, words: &WordSet) -> (u32, u32) {
        let mut cp1 = 0;
        let mut cp2 = 0;
        for word in words {
            if let Ok(cp) = word.parse::<u32>() {
                cp1 = cp;
            }
            if let Ok(cp) = u32::from_str_radix(word, 16) {
                cp2 = cp;
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
        println!("update_searches");
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

fn string_for_codepoint(cp: u32) -> String {
    if cp <= 0xFFFF {
        format!("  {:04X}", cp)
    } else {
        format!("{:>6X}", cp)
    }
}
