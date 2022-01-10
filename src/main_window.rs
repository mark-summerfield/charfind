// Copyright © 2021-22 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::fixed::{
    Action, APPNAME, BUTTON_HEIGHT, BUTTON_WIDTH, ICON, PAD, ROW_HEIGHT,
    WINDOW_HEIGHT_MIN, WINDOW_WIDTH_MIN,
};
use crate::util;
use fltk::prelude::*;

pub struct Widgets {
    pub main_window: fltk::window::Window,
    pub find_combo: fltk::misc::InputChoice,
    pub browser: fltk::browser::HoldBrowser,
    pub copy_input: fltk::input::Input,
}

pub fn make(sender: fltk::app::Sender<Action>) -> Widgets {
    fltk::window::Window::set_default_xclass(APPNAME);
    let (main_window, width) = make_main_window();
    let mut vbox = fltk::group::Flex::default()
        .size_of_parent()
        .with_type(fltk::group::FlexType::Column);
    vbox.set_margin(PAD);
    let (find_combo, top_row) = add_top_row(sender, width);
    vbox.set_size(&top_row, ROW_HEIGHT);
    let browser = add_middle_row(sender, width);
    let (copy_input, bottom_row) = add_bottom_row(sender, width);
    vbox.set_size(&bottom_row, ROW_HEIGHT);
    vbox.end();
    main_window.end();
    Widgets { main_window, find_combo, browser, copy_input }
}

fn make_main_window() -> (fltk::window::Window, i32) {
    let icon = fltk::image::SvgImage::from_data(ICON).unwrap();
    let (x, y, width, height) = get_config_window_rect();
    let mut main_window =
        fltk::window::Window::new(x, y, width, height, APPNAME);
    main_window.set_icon(Some(icon));
    main_window.size_range(
        WINDOW_WIDTH_MIN,
        WINDOW_HEIGHT_MIN,
        fltk::app::screen_size().0 as i32,
        fltk::app::screen_size().1 as i32,
    );
    main_window.make_resizable(true);
    (main_window, width)
}

fn add_top_row(
    sender: fltk::app::Sender<Action>,
    width: i32,
) -> (fltk::misc::InputChoice, fltk::group::Flex) {
    // TODO tooltips to button
    let mut row = fltk::group::Flex::default()
        .with_size(width, ROW_HEIGHT)
        .with_type(fltk::group::FlexType::Row);
    row.set_margin(PAD);
    let mut find_label = fltk::button::Button::default();
    find_label.set_frame(fltk::enums::FrameType::NoBox);
    find_label.visible_focus(false);
    find_label.set_label("&Find:");
    find_label
        .set_align(fltk::enums::Align::Inside | fltk::enums::Align::Right);
    let mut find_combo = fltk::misc::InputChoice::default();
    initialize_find_combo(&mut find_combo, sender);
    util::populate_find_combo(&mut find_combo, sender);
    find_label.set_callback({
        let mut find_combo = find_combo.clone();
        move |_| {
            find_combo.take_focus().unwrap();
        }
    });
    let mut search_button =
        fltk::button::Button::default().with_label("&Search");
    search_button.visible_focus(false);
    search_button.set_callback(move |_| {
        sender.send(Action::Search);
    });
    let mut option_menu_button = fltk::menu::MenuButton::default();
    option_menu_button.set_label("&Options");
    //////////////////////// Help About - Quit
    row.set_size(&find_label, BUTTON_WIDTH - PAD);
    row.set_size(&search_button, BUTTON_WIDTH + (2 * PAD));
    row.set_size(&option_menu_button, BUTTON_WIDTH + (2 * PAD));
    row.end();
    (find_combo, row)
}

fn initialize_find_combo(
    find_combo: &mut fltk::misc::InputChoice,
    sender: fltk::app::Sender<Action>,
) {
    find_combo.menu_button().visible_focus(false);
    find_combo.handle(move |find_combo, event| {
        if !(find_combo.has_focus()
            || find_combo.input().has_focus()
            || find_combo.menu_button().has_focus())
        {
            return false;
        }
        if event == fltk::enums::Event::KeyUp {
            if fltk::app::event_key() == fltk::enums::Key::F2 {
                find_combo.menu_button().popup();
                return true;
            }
            if find_combo.changed() {
                sender.send(Action::Search);
            }
        }
        false
    });
}

fn add_middle_row(
    sender: fltk::app::Sender<Action>,
    width: i32,
) -> fltk::browser::HoldBrowser {
    let mut row = fltk::group::Flex::default()
        .with_size(width, ROW_HEIGHT)
        .with_type(fltk::group::FlexType::Row);
    row.set_margin(PAD);
    let mut browser = fltk::browser::HoldBrowser::default();
    browser.set_column_char('\t');
    browser.handle(move |browser, event| {
        if browser.has_focus()
            && event == fltk::enums::Event::KeyUp
            && fltk::app::event_key().bits() == 32
        {
            sender.send(Action::AddFromTable); // Space
            true
        } else {
            false
        }
    });
    let column = add_copy_buttons(sender);
    row.set_size(&column, BUTTON_WIDTH * 2);
    row.end();
    browser
}

fn add_copy_buttons(
    sender: fltk::app::Sender<Action>,
) -> fltk::group::Flex {
    let mut column = fltk::group::Flex::default()
        .with_type(fltk::group::FlexType::Column);
    column.set_margin(PAD);
    let mut button =
        fltk::button::Button::default().with_label("Add from &Table");
    button.visible_focus(false);
    button.set_callback(move |_| {
        sender.send(Action::AddFromTable);
    });
    column.set_size(&button, BUTTON_HEIGHT);
    let config = CONFIG.get().read().unwrap();
    for (i, c) in config.history.iter().enumerate() {
        let label = format!("&{} Add |{}|", i + 1, c);
        let mut button = fltk::button::Button::default().with_label(&label);
        button.visible_focus(false);
        button.set_callback(move |button| {
            let mut label = button.label();
            label.pop(); // drop '|'
            if let Some(c) = label.pop() {
                sender.send(Action::AddChar(c));
            }
        });
        column.set_size(&button, BUTTON_HEIGHT);
    }
    column.end();
    column
}

fn add_bottom_row(
    sender: fltk::app::Sender<Action>,
    width: i32,
) -> (fltk::input::Input, fltk::group::Flex) {
    // TODO tooltips to buttons
    let mut row = fltk::group::Flex::default()
        .with_size(width, ROW_HEIGHT)
        .with_type(fltk::group::FlexType::Row);
    row.set_margin(PAD);
    let mut copy_button =
        fltk::button::Button::default().with_label("&Copy");
    copy_button.visible_focus(false);
    copy_button.set_callback(move |_| {
        sender.send(Action::Copy);
    });
    let copy_input = fltk::input::Input::default();
    let mut options_button =
        fltk::button::Button::default().with_label("&Options");
    options_button.visible_focus(false);
    options_button.set_callback(move |_| {
        sender.send(Action::Options);
    });
    let mut about_button =
        fltk::button::Button::default().with_label("&About");
    about_button.visible_focus(false);
    about_button.set_callback(move |_| {
        sender.send(Action::About);
    });
    let mut help_button =
        fltk::button::Button::default().with_label("&Help");
    help_button.visible_focus(false);
    help_button.set_callback(move |_| {
        sender.send(Action::Help);
    });
    let mut quit_button =
        fltk::button::Button::default().with_label("&Quit");
    quit_button.visible_focus(false);
    quit_button.set_callback(move |_| {
        sender.send(Action::Quit);
    });
    let width = (width / 6)
        .max((BUTTON_WIDTH as f32 * 1.5) as i32)
        .min(BUTTON_WIDTH);
    row.set_size(&copy_button, width);
    row.set_size(&options_button, width);
    row.set_size(&about_button, width);
    row.set_size(&help_button, width);
    row.set_size(&quit_button, width);
    row.end();
    (copy_input, row)
}

fn get_config_window_rect() -> (i32, i32, i32, i32) {
    let mut config = CONFIG.get().write().unwrap();
    let x = if config.window_x >= 0 {
        config.window_x
    } else {
        util::x() - (config.window_width / 2)
    };
    let y = if config.window_y >= 0 {
        config.window_y
    } else {
        util::y() - (config.window_height / 2)
    };
    if x != config.window_x {
        config.window_x = x;
    }
    if y != config.window_y {
        config.window_y = y;
    }
    (x, y, config.window_width, config.window_height)
}

pub fn add_event_handlers(
    main_window: &mut fltk::window::Window,
    sender: fltk::app::Sender<Action>,
) {
    // Both of these are really needed!
    main_window.set_callback(move |_| {
        if fltk::app::event() == fltk::enums::Event::Close
            || fltk::app::event_key() == fltk::enums::Key::Escape
        {
            sender.send(Action::Quit);
        }
    });
    main_window.handle(move |_, event| match event {
        fltk::enums::Event::KeyUp => match fltk::app::event_key() {
            fltk::enums::Key::Help | fltk::enums::Key::F1 => {
                sender.send(Action::Help);
                true
            }
            fltk::enums::Key::F3 => {
                sender.send(Action::FocusToSearchResults);
                true
            }
            _ => false,
        },
        _ => false,
    });
}
