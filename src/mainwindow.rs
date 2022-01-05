// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::fixed::{
    Action, APPNAME, ICON, PAD, TOOLBAR_HEIGHT, TOOLBUTTON_SIZE,
};
use crate::util;
use fltk::prelude::*;

pub fn make(
    sender: fltk::app::Sender<Action>,
) -> (fltk::window::Window, fltk::frame::Frame) {
    fltk::window::Window::set_default_xclass(APPNAME);
    let icon = fltk::image::SvgImage::from_data(ICON).unwrap();
    let (x, y, width, height) = get_config_window_rect();
    let mut mainwindow =
        fltk::window::Window::new(x, y, width, height, APPNAME);
    mainwindow.set_icon(Some(icon));
    let size = ((TOOLBUTTON_SIZE * 4) / 3) * 6;
    mainwindow.size_range(size, size, size * 4, size * 4);
    mainwindow.make_resizable(true);
    let mut vbox = fltk::group::Flex::default()
        .size_of_parent()
        .with_type(fltk::group::FlexType::Column);
    vbox.set_margin(PAD);
    fltk::frame::Frame::default()
        .with_size(100, 100)
        .with_label("Central Area");
    let statusbar = add_status_bar(&mut vbox, width);
    vbox.end();
    mainwindow.end();
    (mainwindow, statusbar)
}

fn add_status_bar(
    vbox: &mut fltk::group::Flex,
    width: i32,
) -> fltk::frame::Frame {
    let mut status_row = fltk::group::Flex::default()
        .with_size(width, TOOLBUTTON_SIZE)
        .with_type(fltk::group::FlexType::Row);
    let mut statusbar = fltk::frame::Frame::default();
    statusbar.set_frame(fltk::enums::FrameType::EngravedFrame);
    status_row.end();
    vbox.set_size(&status_row, TOOLBUTTON_SIZE);
    statusbar
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
    mainwindow: &mut fltk::window::Window,
    sender: fltk::app::Sender<Action>,
) {
    // Both of these are really needed!
    mainwindow.set_callback(move |_| {
        if fltk::app::event() == fltk::enums::Event::Close
            || fltk::app::event_key() == fltk::enums::Key::Escape
        {
            sender.send(Action::Quit);
        }
    });
    mainwindow.handle(move |_, event| match event {
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
