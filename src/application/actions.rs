// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::fixed::{about_html, HELP_HTML, MESSAGE_DELAY};
use crate::html_form;
use crate::options_form;
use crate::Application;
use fltk::prelude::*;

impl Application {
    pub(crate) fn on_options(&mut self) {
        let form = options_form::Form::default();
        if *form.ok.borrow() {
            self.set_status("options OK", Some(MESSAGE_DELAY));
        } else {
            self.clear_status();
        }
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
        config.save(
            self.main_window.x(),
            self.main_window.y(),
            self.main_window.width(),
            self.main_window.height(),
        );
        self.app.quit();
    }
}
