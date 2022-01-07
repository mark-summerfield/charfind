// Copyright © 2021-22 Mark Summerfield. All rights reserved.
// License: GPLv3

mod actions;
use super::CONFIG;
use crate::fixed::Action;
use crate::html_form;
use crate::main_window;
use fltk::prelude::*;

pub struct Application {
    app: fltk::app::App,
    main_window: fltk::window::Window,
    find_combo: fltk::misc::InputChoice,
    all_radio: fltk::button::RadioRoundButton,
    any_radio: fltk::button::RadioRoundButton,
    table: fltk_table::SmartTable,
    copy_input: fltk::input::Input,
    help_form: Option<html_form::Form>,
    chardata: Option<String>,
    receiver: fltk::app::Receiver<Action>,
}

impl Application {
    pub fn new() -> Self {
        let app =
            fltk::app::App::default().with_scheme(fltk::app::Scheme::Gleam);
        let (sender, receiver) = fltk::app::channel::<Action>();
        let mut widgets = main_window::make(sender);
        main_window::add_event_handlers(&mut widgets.main_window, sender);
        widgets.main_window.show();
        Self {
            app,
            main_window: widgets.main_window,
            find_combo: widgets.find_combo,
            all_radio: widgets.all_radio,
            any_radio: widgets.any_radio,
            table: widgets.table,
            copy_input: widgets.copy_input,
            help_form: None,
            chardata: None,
            receiver,
        }
    }

    pub fn run(&mut self) {
        while self.app.wait() {
            if let Some(action) = self.receiver.recv() {
                match action {
                    Action::Search => self.on_search(),
                    Action::Copy => self.on_copy(),
                    Action::AddChar(c) => self.on_add_char(c),
                    Action::AddFromTable => self.on_add_from_table(),
                    Action::Options => self.on_options(),
                    Action::About => self.on_about(),
                    Action::Help => self.on_help(),
                    Action::Quit => self.on_quit(),
                }
            }
        }
    }
}
