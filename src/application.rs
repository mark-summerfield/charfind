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
    history_menu_button: fltk::menu::MenuButton,
    browser: fltk::browser::HoldBrowser,
    copy_input: fltk::input::Input,
    preview_frame: fltk::frame::Frame,
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
        let mut app = Self {
            app,
            main_window: widgets.main_window,
            find_combo: widgets.find_combo,
            history_menu_button: widgets.history_menu_button,
            browser: widgets.browser,
            copy_input: widgets.copy_input,
            preview_frame: widgets.preview_frame,
            help_form: None,
            chardata: None,
            sender,
            receiver,
        };
        app.on_startup();
        app
    }

    pub fn run(&mut self) {
        while self.app.wait() {
            if let Some(action) = self.receiver.recv() {
                match action {
                    Action::Search => self.on_search(),
                    Action::SearchFor(i) => self.on_search_for(i),
                    Action::Copy => self.on_copy(),
                    Action::Clear => self.copy_input.set_value(""),
                    Action::AddChar(c) => self.on_add_char(c),
                    Action::AddFromTable => self.on_add_from_table(),
                    Action::MaybeAddFromTable => {
                        self.on_maybe_add_from_table()
                    }
                    Action::FocusToSearchResults => {
                        self.browser.take_focus().unwrap_or_default()
                    }
                    Action::PopupSearches => {
                        self.find_combo.take_focus().unwrap_or_default();
                        self.find_combo.menu_button().popup();
                    }
                    Action::UpdatePreview => self.on_update_preview(),
                    Action::Options => self.on_options(),
                    Action::About => self.on_about(),
                    Action::Help => self.on_help(),
                    Action::Quit => self.on_quit(),
                }
            }
        }
    }

    fn on_startup(&mut self) {
        if self.find_combo.menu_button().size() > 0 {
            self.on_search_for(0);
            let mut input = self.find_combo.input();
            input.set_position(0).unwrap_or_default();
            input.set_mark(input.maximum_size()).unwrap_or_default();
            input.take_focus().unwrap_or_default();
        }
    }
}
