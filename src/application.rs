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
    browser: fltk::browser::HoldBrowser,
    copy_input: fltk::input::Input,
    help_form: Option<html_form::Form>,
    chardata: Option<String>,
    sender: fltk::app::Sender<Action>,
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
            browser: widgets.browser,
            copy_input: widgets.copy_input,
            help_form: None,
            chardata: None,
            sender,
            receiver,
        }
    }

    pub fn run(&mut self) {
        while self.app.wait() {
            if let Some(action) = self.receiver.recv() {
                match action {
                    Action::Search => self.on_search(),
                    Action::SearchFor(i) => self.on_search_for(i),
                    Action::Copy => self.on_copy(),
                    Action::AddChar(c) => self.on_add_char(c),
                    Action::AddFromTable => self.on_add_from_table(),
                    Action::FocusToSearchResults => {
                        self.browser.take_focus().unwrap()
                    }
                    Action::Options => self.on_options(),
                    Action::About => self.on_about(),
                    Action::Help => self.on_help(),
                    Action::Quit => self.on_quit(),
                }
            }
        }
    }
}
