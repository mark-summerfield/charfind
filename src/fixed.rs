// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::util::capitalize_first;
use chrono::prelude::*;
use std::env;

pub static APPNAME: &str = "CharFind";
pub static VERSION: &str = "0.1.0";
pub const ICON: &str = include_str!("../images/charfind.svg");
pub const HISTORY_SIZE: usize = 9;
pub const PAD: i32 = 6;
pub const WINDOW_WIDTH_MIN: i32 = 660;
pub const WINDOW_HEIGHT_MIN: i32 = 490;
pub const ROW_HEIGHT: i32 = 40;
pub const BUTTON_HEIGHT: i32 = 30;
pub const BUTTON_WIDTH: i32 = 70;
pub const SCALE_MIN: f32 = 0.5;
pub const SCALE_MAX: f32 = 3.5;
pub const TABLE_ROWS: i32 = 200;
pub const MESSAGE_DELAY: f64 = 10.0; // seconds

#[derive(Copy, Clone, Debug)]
pub enum Action {
    Search,
    Copy,
    AddChar(char),
    AddFromTable,
    Options,
    About,
    Help,
    Quit,
}

pub fn about_html() -> String {
    let year = Local::today().year();
    let year = if year == 2021 {
        format!("{}", year)
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
