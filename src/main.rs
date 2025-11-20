use pancurses::{endwin, initscr};

use sliding_puzzle::{Cell, Level, Menuitems, menu, run_level};

mod levels;

mod init;

fn main() -> Result<(), String>{

    let mut level1 = Level::build_from_tuple(levels::LEVEL1)
        .expect("Built in levels should not fail building");
    let mut level2 = Level::build_from_tuple(levels::LEVEL2)
        .expect("Built in levels should not fail building");
    let mut level3 = Level::build_from_tuple(levels::LEVEL3)
        .expect("Built in levels should not fail building");

    let window = initscr();
    window.keypad(true);
    init::init();

    macro_rules! endwin_return {
        ($val:expr) => {{
            endwin();
            return $val
        }};
    }

    macro_rules! handle {
        ($result:expr) => {{
            match $result {
                Ok(Menuitems::Exit) => endwin_return!(Ok(())),
                Ok(Menuitems::Next) => (),
                Ok(Menuitems::Select) => (),
                Err(e) => endwin_return!(Err(e))
            }   
        }};
    }

    match menu(&window)? {
        Menuitems::Next => (),
        Menuitems::Select => (), //change later
        Menuitems::Exit => endwin_return!(Ok(()))
    };

    handle!(run_level(
        &window, 
        &mut level1
    ));
    handle!(run_level(
        &window, 
        &mut level2
    ));
    handle!(run_level(
        &window, 
        &mut level3, 
    ));
    
    endwin_return!(Ok(()))
}

