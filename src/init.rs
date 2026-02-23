#![allow(unused)]

use pancurses::{COLOR_BLACK, COLOR_GREEN, COLOR_MAGENTA, COLOR_RED, COLOR_WHITE, curs_set, init_color, init_pair, noecho, start_color, use_default_colors};

pub const BLACK: u32 = 0;
pub const RED: u32 = 1;
pub const GREEN: u32 = 2;
pub const WHITE: u32 = 7;

pub fn init() {
    curs_set(0);
    noecho();
    start_color();
    use_default_colors();

    init_pair(BLACK as i16, COLOR_BLACK, -1);
    init_pair(WHITE as i16, COLOR_WHITE, -1);
    init_pair(GREEN as i16, COLOR_GREEN, -1);
    init_pair(RED as i16, COLOR_RED, -1);
}