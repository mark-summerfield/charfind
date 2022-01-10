// Copyright © 2021-22 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::fixed::{
    Action, APPNAME, A_TO_Z, BUTTON_WIDTH, ICON, PAD, ROW_HEIGHT,
    WINDOW_HEIGHT_MIN, WINDOW_WIDTH_MIN,
};
use crate::util;
use fltk::prelude::*;

pub struct Widgets {
    pub main_window: fltk::window::Window,
    pub find_combo: fltk::misc::InputChoice,
    pub history_menu_button: fltk::menu::MenuButton,
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
    let (history_menu_button, copy_input, bottom_row) =
        add_bottom_row(sender, width);
    vbox.set_size(&bottom_row, ROW_HEIGHT);
    vbox.end();
    main_window.end();
    Widgets {
        main_window,
        find_combo,
        history_menu_button,
        browser,
        copy_input,
    }
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
    let mut option_menu_button = fltk::menu::MenuButton::default();
    initialize_option_menu_button(&mut option_menu_button, sender);
    row.set_size(&find_label, BUTTON_WIDTH);
    row.set_size(&option_menu_button, BUTTON_WIDTH);
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

fn initialize_option_menu_button(
    option_menu_button: &mut fltk::menu::MenuButton,
    sender: fltk::app::Sender<Action>,
) {
    option_menu_button.set_label("&Options");
    option_menu_button.add_emit(
        "&Options…",
        fltk::enums::Shortcut::None,
        fltk::menu::MenuFlag::Normal,
        sender,
        Action::Options,
    );
    // TODO divider
    option_menu_button.add_emit(
        "&Help",
        fltk::enums::Shortcut::None,
        fltk::menu::MenuFlag::Normal,
        sender,
        Action::Help,
    );
    option_menu_button.add_emit(
        "&About",
        fltk::enums::Shortcut::None,
        fltk::menu::MenuFlag::Normal,
        sender,
        Action::About,
    );
    // TODO divider
    option_menu_button.add_emit(
        "&Quit",
        fltk::enums::Shortcut::None,
        fltk::menu::MenuFlag::Normal,
        sender,
        Action::Quit,
    );
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
    row.end();
    browser
}

fn add_bottom_row(
    sender: fltk::app::Sender<Action>,
    width: i32,
) -> (fltk::menu::MenuButton, fltk::input::Input, fltk::group::Flex) {
    // TODO tooltips to buttons
    let mut row = fltk::group::Flex::default()
        .with_size(width, ROW_HEIGHT)
        .with_type(fltk::group::FlexType::Row);
    row.set_margin(PAD);
    let mut add_button = fltk::button::Button::default().with_label("&Add");
    add_button.visible_focus(false);
    add_button.set_callback(move |_| {
        sender.send(Action::AddFromTable);
    });
    let copy_input = fltk::input::Input::default();
    let mut copy_button =
        fltk::button::Button::default().with_label("&Copy");
    copy_button.visible_focus(false);
    copy_button.set_callback(move |_| {
        sender.send(Action::Copy);
    });
    let mut history_menu_button =
        fltk::menu::MenuButton::default().with_label("&History");
    populate_history_menu_button(&mut history_menu_button, sender);
    row.set_size(&add_button, BUTTON_WIDTH);
    row.set_size(&copy_button, BUTTON_WIDTH);
    row.set_size(&history_menu_button, BUTTON_WIDTH);
    row.end();
    (history_menu_button, copy_input, row)
}

fn populate_history_menu_button(
    option_menu_button: &mut fltk::menu::MenuButton,
    sender: fltk::app::Sender<Action>,
) {
    option_menu_button.clear();
    let config = CONFIG.get().read().unwrap();
    for (i, c) in config.history.iter().enumerate() {
        option_menu_button.add_emit(
            &format!("&{}  {}", A_TO_Z[i], c),
            fltk::enums::Shortcut::None,
            fltk::menu::MenuFlag::Normal,
            sender,
            Action::AddChar(*c),
        );
    }
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
