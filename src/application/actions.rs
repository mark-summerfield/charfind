// Copyright © 2021-22 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::fixed::{about_html, HELP_HTML, MESSAGE_DELAY};
use crate::html_form;
use crate::options_form;
use crate::Application;
use fltk::prelude::*;

impl Application {
    pub(crate) fn on_search(&mut self) {
        println!("on_search"); // TODO
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
