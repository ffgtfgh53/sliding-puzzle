use pancurses::{endwin, initscr};

use sliding_puzzle::{Cell, Level, run_level};

mod levels;

extern crate pancurses;

fn main() -> Result<(), String>{

    let mut level1 = Level::build_from_tuple(levels::LEVEL1).unwrap();
    let mut level2 = Level::build_from_tuple(levels::LEVEL2).unwrap();
    let window = initscr();
    window.keypad(true);

    run_level(&window, &mut level1)?;
    run_level(&window, &mut level2)?;
    endwin();
    Ok(())
}
