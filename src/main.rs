use std::{error::Error, fs::read_dir};
use std::io;

use pancurses::{endwin, initscr};

use sliding_puzzle::{Level, Menuitems, menu, run_level};

mod init;

const LEVEL_DIRECTORY: &str = "./levels";

fn main() -> Result<(), Box<dyn Error>>{

    let mut level_files = 
        read_dir(LEVEL_DIRECTORY)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    level_files.sort();

    let mut level_vec: Vec<Level> = Vec::with_capacity(level_files.len());

    for filepath in level_files {
        let lev = Level::build_from_file(filepath)?;
        level_vec.push(lev);
    }

    let window = initscr();
    window.keypad(true);
    init::init();

    match menu(&window)? {
        Menuitems::Next => (),
        Menuitems::Select => todo!(),
        Menuitems::Exit => { endwin(); return Ok(());},
    };

    for mut lev in level_vec {
        match run_level(&window,  &mut lev)? {
            Menuitems::Exit => { endwin(); return Ok(());},
            Menuitems::Next => continue,
            Menuitems::Select => todo!()
        }
    }

    endwin();
    Ok(())
}