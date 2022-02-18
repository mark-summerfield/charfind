// Copyright © 2021-22 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::fixed::{
    Action, APPNAME, BUTTON_HEIGHT, BUTTON_WIDTH, ICON, MENU_CHARS, PAD,
    ROW_HEIGHT, WINDOW_HEIGHT_MIN, WINDOW_WIDTH_MIN,
};
use crate::util;
use fltk::{
    app,
    app::Sender,
    browser::HoldBrowser,
    button::Button,
    enums::{Align, Event, FrameType, Key, Shortcut},
    frame::Frame,
    group::Flex,
    image::SvgImage,
    input::Input,
    menu::{MenuButton, MenuFlag},
    misc::InputChoice,
    prelude::*,
    window::Window,
};

pub struct Widgets {
    pub main_window: Window,
    pub find_combo: InputChoice,
    pub history_menu_button: MenuButton,
    pub browser: HoldBrowser,
    pub copy_input: Input,
    pub preview_frame: Frame,
}

pub fn make(sender: Sender<Action>) -> Widgets {
    Window::set_default_xclass(APPNAME);
    let (main_window, width) = make_main_window();
    let mut vbox = Flex::default().column().size_of_parent();
    vbox.set_margin(PAD);
    let (find_combo, history_menu_button, top_row) =
        add_top_row(sender, width);
    vbox.set_size(&top_row, ROW_HEIGHT);
    let (browser, copy_input, preview_frame) =
        add_middle_row(sender, width);
    vbox.end();
    main_window.end();
    Widgets {
        main_window,
        find_combo,
        history_menu_button,
        browser,
        copy_input,
        preview_frame,
    }
}

fn make_main_window() -> (Window, i32) {
    let icon = SvgImage::from_data(ICON).unwrap();
    let (x, y, width, height) = get_config_window_rect();
    let mut main_window = Window::new(x, y, width, height, APPNAME);
    main_window.set_icon(Some(icon));
    main_window.size_range(
        WINDOW_WIDTH_MIN,
        WINDOW_HEIGHT_MIN,
        app::screen_size().0 as i32,
        app::screen_size().1 as i32,
    );
    main_window.make_resizable(true);
    (main_window, width)
}

fn add_top_row(
    sender: Sender<Action>,
    width: i32,
) -> (InputChoice, MenuButton, Flex) {
    let mut row = Flex::default().row().with_size(width, ROW_HEIGHT);
    row.set_margin(PAD);
    let mut find_label = Button::default();
    find_label.set_frame(FrameType::NoBox);
    find_label.visible_focus(false);
    find_label.set_label("&Search:");
    find_label.set_align(Align::Inside | Align::Right);
    let mut find_combo = InputChoice::default();
    initialize_find_combo(&mut find_combo, sender);
    util::populate_find_combo(&mut find_combo, sender);
    find_label.set_callback({
        let mut find_combo = find_combo.clone();
        move |_| {
            find_combo.take_focus().unwrap_or_default();
        }
    });
    row.set_size(&find_label, BUTTON_WIDTH);
    let mut history_menu_button =
        MenuButton::default().with_label("&History");
    history_menu_button.set_tooltip(
        "Add a previously added character to the output editor",
    );
    populate_history_menu_button(&mut history_menu_button, sender);
    row.set_size(&history_menu_button, BUTTON_WIDTH);
    row.end();
    (find_combo, history_menu_button, row)
}

fn initialize_find_combo(
    find_combo: &mut InputChoice,
    sender: Sender<Action>,
) {
    find_combo
        .set_tooltip("Find every 'word' and at least one of 'aword? bword?' but not any '-word's");
    find_combo.menu_button().visible_focus(false);
    find_combo.handle(move |find_combo, event| {
        if !(find_combo.has_focus()
            || find_combo.input().has_focus()
            || find_combo.menu_button().has_focus())
        {
            return false;
        }
        if event == Event::KeyUp && find_combo.changed() {
            sender.send(Action::Search);
        }
        false
    });
}

fn add_middle_row(
    sender: Sender<Action>,
    width: i32,
) -> (HoldBrowser, Input, Frame) {
    let mut row = Flex::default().row().with_size(width, ROW_HEIGHT);
    row.set_margin(PAD);
    let mut browser = HoldBrowser::default();
    browser.set_column_char('\t');
    browser.handle(move |browser, event| {
        if browser.has_focus() {
            if event == Event::KeyUp || event == Event::Released {
                sender.send(Action::UpdatePreview);
            }
            if app::event_is_click()
                && !app::event_inside_widget(&browser.scrollbar())
                && app::event_button() == 1
                && app::event_clicks()
            {
                sender.send(Action::MaybeAddFromTable);
                return true;
            }
        }
        false
    });
    let (copy_input, preview_frame, column) = add_right_column(sender);
    row.set_size(&column, BUTTON_WIDTH);
    row.end();
    (browser, copy_input, preview_frame)
}

fn add_right_column(sender: Sender<Action>) -> (Input, Frame, Flex) {
    let mut column = Flex::default().column();
    add_button(
        "Add the selected character from the table to the output editor",
        "&Add",
        Action::AddFromTable,
        sender,
        &mut column,
    );
    let copy_text = {
        let config = CONFIG.get().read().unwrap();
        config.copy_text.clone()
    };
    let mut copy_input = Input::default();
    copy_input.set_value(&copy_text);
    copy_input.set_tooltip("The output editor: chosen characters are added here and the text here gets copied to the clipboard");
    add_button(
        "Copy the output editor's text to the clipboard",
        "&Copy",
        Action::Copy,
        sender,
        &mut column,
    );
    add_button(
        "Clear the output editor's",
        "C&lear",
        Action::Clear,
        sender,
        &mut column,
    );
    let mut preview_frame = Frame::default();
    let size = preview_frame.label_size();
    preview_frame.set_label_size(size * 3);
    add_button(
        "Pop up the Options dialog",
        "&Options…",
        Action::Options,
        sender,
        &mut column,
    );
    add_button(
        "Show the Help window",
        "Help",
        Action::Help,
        sender,
        &mut column,
    );
    add_button(
        "Pop up the About box",
        "A&bout",
        Action::About,
        sender,
        &mut column,
    );
    Frame::default().with_size(PAD, PAD);
    add_button(
        "Quit the application",
        "&Quit",
        Action::Quit,
        sender,
        &mut column,
    );
    column.set_size(&copy_input, BUTTON_HEIGHT);
    column.set_size(&preview_frame, BUTTON_HEIGHT * 3);
    column.end();
    (copy_input, preview_frame, column)
}

fn add_button(
    tooltip: &str,
    label: &str,
    action: Action,
    sender: Sender<Action>,
    column: &mut Flex,
) {
    let mut button = Button::default().with_label(label);
    button.set_tooltip(tooltip);
    button.visible_focus(false);
    button.set_callback(move |_| {
        sender.send(action);
    });
    column.set_size(&button, BUTTON_HEIGHT);
}

pub(crate) fn populate_history_menu_button(
    history_menu_button: &mut MenuButton,
    sender: Sender<Action>,
) {
    history_menu_button.clear();
    let config = CONFIG.get().read().unwrap();
    let size = config.history_size;
    let base = if (10..=26).contains(&size) { 9 } else { 0 };
    for (i, c) in config.history.iter().enumerate() {
        if i == size {
            break;
        }
        history_menu_button.add_emit(
            &format!("&{}  {c}", MENU_CHARS[base + i]),
            Shortcut::None,
            MenuFlag::Normal,
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
    main_window: &mut Window,
    sender: Sender<Action>,
) {
    // Both of these are really needed!
    main_window.set_callback(move |_| {
        if app::event() == Event::Close || app::event_key() == Key::Escape {
            sender.send(Action::Quit);
        }
    });
    main_window.handle(move |_, event| match event {
        Event::KeyUp => match app::event_key() {
            Key::Help | Key::F1 => {
                sender.send(Action::Help);
                true
            }
            Key::F2 => {
                sender.send(Action::PopupSearches);
                true
            }
            Key::F3 => {
                sender.send(Action::FocusToSearchResults);
                true
            }
            _ => false,
        },
        _ => false,
    });
}
