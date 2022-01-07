// Copyright © 2021-22 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::fixed::{
    Action, APPNAME, BUTTON_HEIGHT, BUTTON_WIDTH, HISTORY_SIZE, ICON, PAD,
    ROW_HEIGHT, WINDOW_HEIGHT_MIN, WINDOW_WIDTH_MIN,
};
use crate::util;
use fltk::prelude::*;

pub struct Widgets {
    pub main_window: fltk::window::Window,
    pub find_combo: fltk::misc::InputChoice,
    pub all_radio: fltk::button::RadioRoundButton,
    pub any_radio: fltk::button::RadioRoundButton,
    pub browser: fltk::browser::HoldBrowser,
    pub copy_input: fltk::input::Input,
}

pub fn make(sender: fltk::app::Sender<Action>) -> Widgets {
    fltk::window::Window::set_default_xclass(APPNAME);
    let icon = fltk::image::SvgImage::from_data(ICON).unwrap();
    let (x, y, width, height) = get_config_window_rect();
    let mut main_window =
        fltk::window::Window::new(x, y, width, height, APPNAME);
    main_window.set_icon(Some(icon));
    main_window.size_range(
        WINDOW_WIDTH_MIN,
        WINDOW_HEIGHT_MIN,
        i32::MAX,
        i32::MAX,
    );
    main_window.make_resizable(true);
    let mut vbox = fltk::group::Flex::default()
        .size_of_parent()
        .with_type(fltk::group::FlexType::Column);
    vbox.set_margin(PAD);
    let (find_combo, all_radio, any_radio, top_row) =
        add_top_row(sender, width);
    vbox.set_size(&top_row, ROW_HEIGHT);
    let browser = add_middle_row(sender, width);
    let (copy_input, bottom_row) = add_bottom_row(sender, width);
    vbox.set_size(&bottom_row, ROW_HEIGHT);
    vbox.end();
    main_window.end();
    Widgets {
        main_window,
        find_combo,
        all_radio,
        any_radio,
        browser,
        copy_input,
    }
}

fn add_top_row(
    sender: fltk::app::Sender<Action>,
    width: i32,
) -> (
    fltk::misc::InputChoice,
    fltk::button::RadioRoundButton,
    fltk::button::RadioRoundButton,
    fltk::group::Flex,
) {
    // TODO tooltips to button & radio buttons
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
    initialize_find_combo(&mut find_combo);
    find_label.set_callback({
        let mut find_combo = find_combo.clone();
        move |_| {
            find_combo.take_focus().unwrap();
        }
    });
    let mut search_button =
        fltk::button::Button::default().with_label("&Search");
    search_button.set_callback(move |_| {
        sender.send(Action::Search);
    });
    let match_label = fltk::frame::Frame::default()
        .with_label("Match:")
        .with_align(fltk::enums::Align::Inside | fltk::enums::Align::Right);
    let mut all_radio =
        fltk::button::RadioRoundButton::default().with_label("A&ll Words");
    all_radio.set(true);
    let any_radio =
        fltk::button::RadioRoundButton::default().with_label("A&ny Words");
    let label_width = BUTTON_WIDTH - PAD;
    let radio_width = (BUTTON_WIDTH as f32 * 1.5) as i32;
    let width = (width / 6).max(radio_width).min(label_width);
    row.set_size(&find_label, width.min(label_width));
    row.set_size(&search_button, width.min(BUTTON_WIDTH));
    row.set_size(&match_label, width.min(label_width));
    row.set_size(&all_radio, width.max(radio_width));
    row.set_size(&any_radio, width.max(radio_width));
    row.end();
    (find_combo, all_radio, any_radio, row)
}

fn initialize_find_combo(find_combo: &mut fltk::misc::InputChoice) {
    let config = CONFIG.get().read().unwrap();
    for i in 0..HISTORY_SIZE {
        find_combo.add(&config.searches[i]);
    }
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
    let small_width = (width / 6).max(BUTTON_WIDTH);
    // TODO this works in the example but not here!
    //let widths = &[small_width, small_width, width - 2 * small_width];
    //browser.set_column_widths(widths);
    browser.set_column_char('\t');
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
    for i in 0..HISTORY_SIZE {
        let label = format!("&{} Add |{}|", i + 1, config.history[i]);
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
    // TODO tooltips to button & radio buttons
    let mut row = fltk::group::Flex::default()
        .with_size(width, ROW_HEIGHT)
        .with_type(fltk::group::FlexType::Row);
    row.set_margin(PAD);
    let mut copy_button =
        fltk::button::Button::default().with_label("&Copy");
    copy_button.set_callback(move |_| {
        sender.send(Action::Copy);
    });
    let copy_input = fltk::input::Input::default();
    let mut options_button =
        fltk::button::Button::default().with_label("&Options");
    options_button.set_callback(move |_| {
        sender.send(Action::Options);
    });
    let mut about_button =
        fltk::button::Button::default().with_label("&About");
    about_button.set_callback(move |_| {
        sender.send(Action::About);
    });
    let mut help_button =
        fltk::button::Button::default().with_label("&Help");
    help_button.set_callback(move |_| {
        sender.send(Action::Help);
    });
    let mut quit_button =
        fltk::button::Button::default().with_label("&Quit");
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
            _ => false,
        },
        _ => false,
    });
}
