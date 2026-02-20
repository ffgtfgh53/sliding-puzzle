use std::fs::read_dir;
use std::io;

use pancurses::{endwin, initscr};

use sliding_puzzle::{Level, Menuitems, menu, run_level};

mod init;

const LEVEL_DIRECTORY: &str = "levels";

// fn write_to_file() -> Result<(), String>{
//     // Write the levels to a file
//     LevelBuilder::from_tuple(levels::LEVEL1).write_to_file("level1.json".to_string())?;
//     LevelBuilder::from_tuple(levels::LEVEL2).write_to_file("level2.json".to_string())?;
//     LevelBuilder::from_tuple(levels::LEVEL3).write_to_file("level3.json".to_string())?;
//     Ok(())
// }

fn main() -> Result<(), String>{

    let mut level_files = 
        read_dir(LEVEL_DIRECTORY)
        .map_err(|e| e.to_string())?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .map_err(|e| e.to_string())?;

    level_files.sort();

    let mut level_vec: Vec<Level> = Vec::with_capacity(level_files.len());

    for filepath in level_files {
        let Ok(lev) = Level::build_from_file(filepath) else { continue };
        level_vec.push(lev);
    }

    // let mut level1: Level= Level::build_from_builder(LevelBuilder::from_file("level1.json".to_string()).expect("work")).expect("Built in");

    // let mut level1 = Level::build_from_tuple(levels::LEVEL1)
        // .expect("Built in levels should not fail building");
    // let mut level2 = Level::build_from_tuple(levels::LEVEL2)
    //     .expect("Built in levels should not fail building");
    // let mut level3 = Level::build_from_tuple(levels::LEVEL3)
    //     .expect("Built in levels should not fail building");

    let window = initscr();
    window.keypad(true);
    init::init();

    // macro_rules! endwin_return {
    //     ($val:expr) => {{
    //         endwin();
    //         return $val
    //     }};
    // }
    //
    // macro_rules! handle {
    //     ($result:expr) => {{
    //         match $result {
    //             Ok(Menuitems::Exit) => endwin_return!(Ok(())),
    //             Ok(Menuitems::Next) => (),
    //             Ok(Menuitems::Select) => (),
    //             Err(e) => endwin_return!(Err(e))
    //         }   
    //     }};
    // }

    match menu(&window)? {
        Menuitems::Next => (),
        Menuitems::Select => (), //change later
        Menuitems::Exit => { endwin(); return Ok(());},
    };

    for mut lev in level_vec {
        match run_level(&window,  &mut lev)? {
            Menuitems::Exit => { endwin(); return Ok(());},
            Menuitems::Next => continue,
            Menuitems::Select => todo!()
        }
    }

    // handle!(run_level(
    //     &window, 
    //     &mut level1
    // ));
    // handle!(run_level(
    //     &window, 
    //     &mut level2
    // ));
    // handle!(run_level(
    //     &window, 
    //     &mut level3, 
    // ));
    
    endwin();
    Ok(())
}