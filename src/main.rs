use pancurses::{curs_set, endwin, initscr, noecho};

use sliding_puzzle::{Cell, Level, run_level};

mod levels;

extern crate pancurses;

fn main() -> Result<(), String>{

    let mut level1 = Level::build_from_tuple(levels::LEVEL1).unwrap();
    let mut level2 = Level::build_from_tuple(levels::LEVEL2).unwrap();
    let mut level3 = Level::build_from_tuple(levels::LEVEL3).unwrap();

    let window = initscr();
    window.keypad(true);
    curs_set(0);
    noecho();

    run_level(&window, &mut level1, &"Level 1".to_string()).map_err(|e| {endwin(); e})?;
    run_level(&window, &mut level2, &"Level 2".to_string()).map_err(|e| {endwin(); e})?;
    run_level(&window, &mut level3, &"Level 3".to_string()).map_err(|e| {endwin(); e})?;
    endwin();
    Ok(())
}
