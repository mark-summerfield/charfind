// Copyright © 2021-22 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::fixed::{about_html, Action, CHARDATA, HELP_HTML};
use crate::html_form;
use crate::main_window;
use crate::options_form;
use crate::util;
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
        let (all_of, any_of, none_of) = self.get_search_sets();
        if all_of.is_empty() && any_of.is_empty() {
            return; // nothing to search for
        }
        self.browser.clear();
        let (cp1, cp2) =
            self.get_code_points(&all_of.union(&any_of).collect());
        self.maybe_populate_chardata();
        let mut n = 1;
        if let Some(chardata) = &self.chardata {
            for line in chardata.lines() {
                let (cp, desc, keywords) = self.get_unicode_data(line);
                if (cp != 0 && (cp == cp1 || cp == cp2))
                    || (keywords.intersection(&none_of).count() == 0
                        && (any_of.is_empty()
                            || keywords.intersection(&any_of).count() > 0)
                        && keywords.intersection(&all_of).count()
                            == all_of.len())
                {
                    if let Some(c) = char::from_u32(cp) {
                        n += 1;
                        let bg = if n % 2 == 0 { "@B247" } else { "" };
                        let cp = util::string_for_codepoint(cp);
                        let desc = desc.to_lowercase();
                        self.browser.insert(
                            n,
                            &format!(
                                "{bg}@F{}@.{c}\t{cp}\t{desc}",
                                self.browser_font_index
                            ),
                        );
                    }
                }
            }
        }
        self.add_header_line(n - 1);
    }

    fn add_header_line(&mut self, n: i32) {
        if n > 0 {
            let s = if n > 1 { "es" } else { "" };
            self.browser.insert(
                1,
                &format!(
                    "@C7@B58@F{}@.Char\tU+HHHH\tDescription ({} match{s})",
                    self.browser_font_index,
                    n.separate_with_commas(),
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

    fn get_search_sets(&self) -> (WordSet, WordSet, WordSet) {
        if let Some(line) = self.find_combo.value() {
            let mut all_of = WordSet::new();
            let mut any_of = WordSet::new();
            let mut none_of = WordSet::new();
            for word in line.split_whitespace() {
                if let Some(word) = word.strip_suffix('?') {
                    any_of.insert(word.to_uppercase());
                } else if let Some(word) = word.strip_prefix('-') {
                    none_of.insert(word.to_uppercase());
                } else {
                    all_of.insert(word.to_uppercase());
                }
            }
            (all_of, any_of, none_of)
        } else {
            (WordSet::new(), WordSet::new(), WordSet::new())
        }
    }

    fn get_code_points(&self, words: &HashSet<&String>) -> (u32, u32) {
        let mut cp1 = 0;
        let mut cp2 = 0;
        for word in words {
            if word.starts_with('+') || word.starts_with('-') {
                continue; // + and - mean required or optional in charfind
            }
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
        let cp = u32::from_str_radix(cols[0], 16).unwrap_or(0);
        let keywords = cols[2]
            .split('\x0B') // \v VT
            .map(|s| s.to_owned())
            .collect::<WordSet>();
        (cp, cols[1].to_string(), keywords)
    }

    fn update_searches(&mut self) {
        if let Some(line) = self.find_combo.value() {
            if util::add_to_searches(&line) {
                util::populate_find_combo(
                    &mut self.find_combo,
                    self.sender,
                );
            }
        }
    }

    pub(crate) fn on_search_for(&mut self, i: i32) {
        if let Some(item) = self.find_combo.menu_button().at(i) {
            if let Some(text) = item.label() {
                let (_, term) = text.split_at(3);
                self.find_combo.set_value(term);
                self.sender.send(Action::Search);
            }
        }
    }

    pub(crate) fn on_update_preview(&mut self) {
        self.preview_frame.set_label("");
        if let Some(c) = self.get_selected_char() {
            self.preview_frame.set_label(&c.to_string());
        }
    }

    fn get_selected_char(&mut self) -> Option<char> {
        if let Some(text) = self.browser.selected_text() {
            let parts: Vec<&str> = text.split('\t').collect();
            if !parts.is_empty() {
                let field = parts[0];
                if field.ends_with("Char") {
                    return None; // Title row
                }
                if !field.is_empty() {
                    return field.chars().last();
                }
            }
        }
        None
    }

    pub(crate) fn on_copy(&mut self) {
        let text = self.copy_input.value();
        if !text.is_empty() {
            fltk::app::copy(&text);
        }
    }

    pub(crate) fn on_add_char(&mut self, c: char) {
        util::add_to_history(c);
        main_window::populate_history_menu_button(
            &mut self.history_menu_button,
            self.sender,
        );
        let mut text = self.copy_input.value();
        text.push(c);
        self.copy_input.set_value(&text);
    }

    pub(crate) fn on_add_from_table(&mut self) {
        if let Some(c) = self.get_selected_char() {
            self.on_add_char(c);
        }
    }

    pub(crate) fn on_maybe_add_from_table(&mut self) {
        if let Some(c) = self.get_selected_char() {
            let text = self.copy_input.value();
            if !text.ends_with(c) {
                self.on_add_char(c);
            }
        }
    }

    pub(crate) fn on_options(&mut self) {
        let form = options_form::Form::default();
        if *form.ok.borrow() {
            util::populate_find_combo(&mut self.find_combo, self.sender);
            main_window::populate_history_menu_button(
                &mut self.history_menu_button,
                self.sender,
            );
        }
    }

    pub(crate) fn on_about(&mut self) {
        html_form::Form::new("About", &about_html(), true, 500, 280, false);
    }

    pub(crate) fn on_help(&mut self) {
        if let Some(help_form) = &mut self.help_form {
            help_form.show();
        } else {
            self.help_form = Some(html_form::Form::new(
                "Help", HELP_HTML, false, 640, 480, true,
            ));
        }
    }

    pub(crate) fn on_quit(&mut self) {
        let config = CONFIG.get().read().unwrap();
        config.save(
            self.main_window.x(),
            self.main_window.y(),
            self.main_window.width(),
            self.main_window.height(),
            &self.copy_input.value(),
        );
        self.app.quit();
    }
}
