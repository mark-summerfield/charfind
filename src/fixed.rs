// Copyright © 2021-22 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::util::capitalize_first;
use chrono::prelude::*;
use fltk::app;
use std::env;

pub static APPNAME: &str = "CharFind";
pub static VERSION: &str = "1.2.5";
pub static CHARDATA: &[u8] = include_bytes!("../data/chardata.txt.gz");
pub static HELP_HTML: &str = include_str!("../data/help.html");
pub const ICON: &str = include_str!("../images/charfind.svg");
pub const AUTO_MENU_SIZE: usize = 26;
pub const PAD: i32 = 6;
pub const WINDOW_WIDTH_MIN: i32 = 400;
pub const WINDOW_HEIGHT_MIN: i32 = 440;
pub const ROW_HEIGHT: i32 = 40;
pub const BUTTON_HEIGHT: i32 = 30;
pub const BUTTON_WIDTH: i32 = 90;
pub const SCALE_MIN: f32 = 0.5;
pub const SCALE_MAX: f32 = 3.5;
pub static MENU_CHARS: [char; 35] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E',
    'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[derive(Copy, Clone, Debug)]
pub enum Action {
    About,
    AddChar(char),
    AddFromTable,
    MaybeAddFromTable,
    Copy,
    Clear,
    FocusToSearchResults,
    Help,
    Options,
    Quit,
    Search,
    PopupSearches,
    SearchFor(i32),
    UpdatePreview,
}

pub fn about_html() -> String {
    let year = Local::today().year();
    let year = if year == 2022 {
        year.to_string()
    } else {
        format!("2022-{}", year - 2000)
    };
    format!(
        "<p><center><font size=6 color=navy><b>{}</b> v{}</font>
</center></p>
<p><center><font color=navy size=5>An application for finding Unicode
characters.</font></center></p>
<p><center><font size=4>
<a href=\"https://github.com/mark-summerfield/charfind\">https://github.com/mark-summerfield/charfind</a>
</font></center></p>
<p><center>
<font size=4 color=green>
Copyright © {} Mark Summerfield.<br>
All rights reserved.<br>
License: GPLv3.</font>
</center></p>
<p><center><font size=4 color=#555>
Rust {} • fltk-rs {} • FLTK {} • {}/{}
</font></center></p>",
        APPNAME,
        VERSION,
        year,
        rustc_version_runtime::version(),
        app::crate_version(),
        app::version_str(),
        capitalize_first(env::consts::OS),
        env::consts::ARCH
    )
}
