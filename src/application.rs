// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

mod actions;
use super::CONFIG;
use crate::fixed::Action;
use crate::html_form;
use crate::mainwindow;
use fltk::prelude::*;

pub struct Application {
    app: fltk::app::App,
    mainwindow: fltk::window::Window,
    statusbar: fltk::frame::Frame,
    helpform: Option<html_form::Form>,
    receiver: fltk::app::Receiver<Action>,
}

impl Application {
    pub fn new() -> Self {
        let app =
            fltk::app::App::default().with_scheme(fltk::app::Scheme::Gleam);
        let (sender, receiver) = fltk::app::channel::<Action>();
        let (mut mainwindow, statusbar) = mainwindow::make(sender);
        mainwindow::add_event_handlers(&mut mainwindow, sender);
        mainwindow.show();
        let mut app = Self {
            app,
            mainwindow,
            statusbar,
            helpform: None,
            receiver,
        };
        app
    }

    pub fn run(&mut self) {
        while self.app.wait() {
            if let Some(action) = self.receiver.recv() {
                match action {
                    Action::Search => println!("Search"),
                    Action::Copy => println!("Copy"),
                    Action::Options => self.on_options(),
                    Action::About => self.on_about(),
                    Action::Help => self.on_help(),
                    Action::Quit => self.on_quit(),
                }
            }
        }
    }

    fn set_status(&mut self, message: &str, timeout: Option<f64>) {
        self.statusbar.set_label(message);
        fltk::app::redraw(); // redraws the world
        if let Some(timeout) = timeout {
            fltk::app::add_timeout(timeout, {
                let mut statusbar = self.statusbar.clone();
                move || {
                    statusbar.set_label("");
                    fltk::app::redraw(); // redraws the world
                }
            });
        }
    }

    fn clear_status(&mut self) {
        self.statusbar.set_label("");
        fltk::app::redraw(); // redraws the world
    }
}
