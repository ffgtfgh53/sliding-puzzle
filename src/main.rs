use std::{thread::sleep, time::Duration};

use pancurses::{Input, endwin, initscr};

use sliding_puzzle::{Cell, Dir, Level};
mod levels;

extern crate pancurses;

fn main() {

    let mut level1 = Level::build_from_tuple(levels::LEVEL1).unwrap();
    let window = initscr();
    window.keypad(true);


    let mut dir: Option<Dir>;
    window.addstr(level1.to_string());
    window.refresh();
    while !level1.is_done() {
        // window.addstr();
        match window.getch() {
            Some(Input::Character('q')) => break,
            Some(Input::KeyUp) => dir = Some(Dir::Up),
            Some(Input::KeyDown) => dir = Some(Dir::Down),
            Some(Input::KeyLeft) => dir = Some(Dir::Left),
            Some(Input::KeyRight) => dir = Some(Dir::Right),
            _ => continue,
        };
        level1.player_state = dir;
        loop {
            if let None = level1.player_state {break;}
            level1.tick();
            window.clear();
            window.addstr(level1.to_string());
            window.refresh();
            sleep(Duration::from_millis(50));
        }
        window.refresh();
    }
    endwin();
}
