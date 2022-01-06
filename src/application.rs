// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

mod actions;
use super::CONFIG;
use crate::fixed::Action;
use crate::html_form;
use crate::main_window;
use fltk::prelude::*;
use fltk_table::SmartTable;

pub struct Application {
    app: fltk::app::App,
    main_window: fltk::window::Window,
    find_combo: fltk::misc::InputChoice,
    all_radio: fltk::button::RadioRoundButton,
    any_radio: fltk::button::RadioRoundButton,
    table: fltk_table::SmartTable,
    copy_input: fltk::input::Input,
    status_bar: fltk::frame::Frame,
    help_form: Option<html_form::Form>,
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
            status_bar: widgets.status_bar,
            help_form: None,
            receiver,
        }
    }

    pub fn run(&mut self) {
        while self.app.wait() {
            if let Some(action) = self.receiver.recv() {
                match action {
                    Action::Search => println!("Search"),
                    Action::Copy => println!("Copy"), // copy copy_input to clipboard
                    Action::AddChar(c) => {
                        println!("AddChar: {}", c) // add to copy_input
                    }
                    Action::AddFromTable => println!("AddFromTable"), // add to copy_input
                    Action::Options => self.on_options(),
                    Action::About => self.on_about(),
                    Action::Help => self.on_help(),
                    Action::Quit => self.on_quit(),
                }
            }
        }
    }

    fn set_status(&mut self, message: &str, timeout: Option<f64>) {
        self.status_bar.set_label(message);
        fltk::app::redraw(); // redraws the world
        if let Some(timeout) = timeout {
            fltk::app::add_timeout(timeout, {
                let mut status_bar = self.status_bar.clone();
                move || {
                    status_bar.set_label("");
                    fltk::app::redraw(); // redraws the world
                }
            });
        }
    }

    fn clear_status(&mut self) {
        self.status_bar.set_label("");
        fltk::app::redraw(); // redraws the world
    }
}
