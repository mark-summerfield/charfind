// Copyright © 2021-22 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::util::capitalize_first;
use chrono::prelude::*;
use std::env;

pub static APPNAME: &str = "CharFind";
pub static VERSION: &str = "0.1.0";
pub static CHARDATA: &[u8] = include_bytes!("../chardata.txt.gz");
pub const ICON: &str = include_str!("../images/charfind.svg");
pub const AUTO_MENU_SIZE: usize = 26;
pub const PAD: i32 = 6;
pub const WINDOW_WIDTH_MIN: i32 = 400;
pub const WINDOW_HEIGHT_MIN: i32 = 240;
pub const ROW_HEIGHT: i32 = 40;
pub const BUTTON_HEIGHT: i32 = 30;
pub const BUTTON_WIDTH: i32 = 90;
pub const SCALE_MIN: f32 = 0.5;
pub const SCALE_MAX: f32 = 3.5;
pub static A_TO_Z: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
    'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[derive(Copy, Clone, Debug)]
pub enum Action {
    Search,
    SearchFor(i32),
    Copy,
    AddChar(char),
    AddFromTable,
    FocusToSearchResults,
    Options,
    About,
    Help,
    Quit,
}

pub fn about_html() -> String {
    let year = Local::today().year();
    let year = if year == 2021 {
        year.to_string()
    } else {
        format!("2021-{}", year - 2000)
    };
    format!(
        "<p><center><font size=6 color=navy><b>{}</b> v{}</font>
</center></p>
<p><center><font size=4>
<a href=\"http://www.qtrac.eu/charfind.html\">www.qtrac.eu/charfind.html</a>
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
        fltk::app::crate_version(),
        fltk::app::version_str(),
        capitalize_first(env::consts::OS),
        env::consts::ARCH
    )
}

// NOTE
// The Find text consists of one or more required words, plain words,
// unwanted words, and code points.
// For example: +left math symbol -greek 2026
// This means match any characters whose description contains left and math
// or symbol (or both) but not greek, or any character whose code point is
// either 2026 decimal or U+07EA (0x07EA is 2026 decimal).
// In general it should work as expected: use +words for required, plain
// words for optional (but at least one of these must match), -words for
// unwanted. And to look up by codepoint just enter the decimal or
// hexadecimal value.

pub static HELP_HTML: &str = "<body>
<p><center><font color=navy size=6><b>CharFind</b></font></center></p>
<font color=blue size=5>The purpose of the game is to remove all the
tiles.</font>
<p>
<font color=#008000 size=4>
Click a tile that has at least one vertically or horizontally adjoining tile
of the same color to remove it and any vertically or horizontally adjoining
tiles of the same color, and <i>their</i> vertically or horizontally
adjoining tiles, and so on. <i>(So clicking a tile with no adjoining tiles
of the same color does nothing.)</i> The more tiles that are removed in one
go, the higher the score.
</font>
</p>
<table border=1 align=center>
<font color=blue>
<tr><th>Key</th><th>Action</th></tr>
<tr><td><b>a</b></td><td>Show About box</td></tr>
<tr><td><b>h</b> or <b>F1</b></td><td>Show this Help window</td></tr>
<tr><td><b>n</b></td><td>New Game</td></tr>
<tr><td><b>o</b></td><td>View or Edit Options</td></tr>
<tr><td><b>q</b> or <b>Esc</b></td><td>Quit</td></tr>
<tr><td><b>←</b></td><td>Move the focus left</td></tr>
<tr><td><b>→</b></td><td>Move the focus right</td></tr>
<tr><td><b>↑</b></td><td>Move the focus up</td></tr>
<tr><td><b>↓</b></td><td>Move the focus down</td></tr>
<tr><td><b>Space</b></td><td>Click the focused tile</td></tr>
</font>
</table>
</body>";
