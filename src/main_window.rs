// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::fixed::{
    Action, APPNAME, BUTTON_WIDTH, ICON, PAD, TOOLBAR_HEIGHT,
    TOOLBUTTON_SIZE,
};
use crate::util;
use fltk::prelude::*;

pub struct Widgets {
    pub main_window: fltk::window::Window,
    pub find_combo: fltk::misc::InputChoice,
    pub status_bar: fltk::frame::Frame,
    pub all_radio: fltk::button::RadioRoundButton,
    pub any_radio: fltk::button::RadioRoundButton,
}

pub fn make(sender: fltk::app::Sender<Action>) -> Widgets {
    fltk::window::Window::set_default_xclass(APPNAME);
    let icon = fltk::image::SvgImage::from_data(ICON).unwrap();
    let (x, y, width, height) = get_config_window_rect();
    let mut main_window =
        fltk::window::Window::new(x, y, width, height, APPNAME);
    main_window.set_icon(Some(icon));
    let size = ((TOOLBUTTON_SIZE * 4) / 3) * 6;
    main_window.size_range(size, size, i32::MAX, i32::MAX);
    main_window.make_resizable(true);
    let mut vbox = fltk::group::Flex::default()
        .size_of_parent()
        .with_type(fltk::group::FlexType::Column);
    vbox.set_margin(PAD);
    let (find_combo, all_radio, any_radio, top_row) =
        add_top_row(sender, width);
    vbox.set_size(&top_row, TOOLBAR_HEIGHT);
    fltk::frame::Frame::default()
        .with_size(100, 100)
        .with_label("Central Area");
    let status_bar = add_status_bar(&mut vbox, width);
    vbox.end();
    main_window.end();
    Widgets { main_window, find_combo, all_radio, any_radio, status_bar }
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
        .with_size(width, TOOLBAR_HEIGHT)
        .with_type(fltk::group::FlexType::Row);
    row.set_margin(PAD);
    let mut find_label = fltk::button::Button::default();
    find_label.set_frame(fltk::enums::FrameType::NoBox);
    find_label.visible_focus(false);
    find_label.set_label("&Find:");
    find_label
        .set_align(fltk::enums::Align::Inside | fltk::enums::Align::Right);
    let find_combo = fltk::misc::InputChoice::default();
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
    row.set_size(&search_button, width.min(label_width));
    row.set_size(&match_label, width.min(label_width));
    row.set_size(&all_radio, width.max(radio_width));
    row.set_size(&any_radio, width.max(radio_width));
    row.end();
    (find_combo, all_radio, any_radio, row)
}

fn add_status_bar(
    vbox: &mut fltk::group::Flex,
    width: i32,
) -> fltk::frame::Frame {
    let mut status_row = fltk::group::Flex::default()
        .with_size(width, TOOLBUTTON_SIZE)
        .with_type(fltk::group::FlexType::Row);
    let mut status_bar = fltk::frame::Frame::default();
    status_bar.set_frame(fltk::enums::FrameType::EngravedFrame);
    status_row.end();
    vbox.set_size(&status_row, TOOLBUTTON_SIZE);
    status_bar
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
