use std::{thread::sleep, time::Duration, usize};

use pancurses::{Attribute, COLOR_PAIR, Input, Window, endwin, resize_term};

use crate::init::{BLACK, GREEN, RED};

mod init;

// pos: position
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell { Empty, Wall, OutOfBounds, Player, Goal }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dir { Up, Down, Left, Right }

#[derive(Debug)]
pub struct Level<const LENY: usize, const LENX: usize> {
    start_pos: [i8; 2],
    current_pos: [i8; 2],
    goal_pos: [i8; 2],
    layout: [[Cell; LENX]; LENY],
    pub player_state: Option<Dir>,
    redraw_goal: Option<[i8; 2]>, //need to improve, simply check goal_pos
    size: [usize; 2],
}

impl<const LENY: usize, const LENX: usize> Level<LENY, LENX> {
    pub fn start_pos(&self) -> &[i8; 2] {&self.start_pos}
    pub fn current_pos(&self) -> &[i8; 2] {&self.current_pos}
    pub fn goal_pos(&self) -> &[i8; 2] {&self.goal_pos}
    pub fn layout(&self) -> &[[Cell; LENX]; LENY] {&self.layout}
    pub fn size(&self) -> &[usize; 2] {&self.size}

    fn new(
        &start_pos: &[i8; 2], 
        &goal_pos: &[i8; 2], 
        layout: [[Cell; LENX]; LENY]
    ) -> Level<LENY, LENX>{
        // take ownership of layout(procesed)
        Level {start_pos, 
            current_pos: start_pos, 
            goal_pos, layout, 
            player_state: None, 
            redraw_goal: None, 
            size: [LENY, LENX]
        }
    }

    pub fn build(
        &start_pos: &[i8; 2], 
        &goal_pos: &[i8; 2], 
        layout: &[[Cell; LENX]; LENY],
    ) -> Result<Level<LENY,LENX>, String> {
        let size: [i8; 2] = [LENY.try_into().map_err(|_| "Len too big")?, 
                                LENX.try_into().map_err(|_| "Len too big")?];
        Self::check_pos_valid_from_size(&start_pos, &size)?;
        Self::check_pos_valid_from_size(&goal_pos, &size)?;
        if Self::get_cell_from_layout(&layout, &start_pos) != Cell::Empty { 
            return Err("Invalid start pos: start pos must be empty".to_string())
        }
        if Self::get_cell_from_layout(&layout, &goal_pos) != Cell::Empty { 
            return Err("Invalid goal pos: goal pos must be empty".to_string())
        }
        let mut processed = layout.clone();
        processed[start_pos[0] as usize][start_pos[1] as usize] = Cell::Player;
        processed[goal_pos[0] as usize][goal_pos[1] as usize] = Cell::Goal;
        // not complete checks, but checking if solvable requires solving it
        // will at least prevent panicking due to invalid pos
        Ok(Level::new(&start_pos, &goal_pos, processed))
    }

    pub fn build_from_tuple(
        (&start_pos, &goal_pos, layout, ): 
        (&[i8; 2], &[i8; 2], &[[Cell; LENX]; LENY])
    ) -> Result<Level<LENY, LENX>, String> {
            Self::build(&start_pos, &goal_pos, &layout)
        }

    pub fn is_pos_valid(&pos: &[i8; 2]) -> bool {
        let [y, x] = pos;
        (..LENY).contains(&(y as usize)) && (..LENX).contains(&(x as usize))
    }

    pub fn check_pos_valid_from_size(&pos: &[i8; 2], &size: &[i8; 2]
    ) -> Result<(), String> {
        let ([may, max], [y, x]) = (size, pos);
        if (..may).contains(&y) && (..max).contains(&x) {Ok(())}
        else {Err(format!("Invalid position: {:?}", pos))}
    }

    fn get_cell_from_layout(layout: &[[Cell; LENX]; LENY], &pos: &[i8; 2]
    ) -> Cell {
        let [a, b] = pos;
        layout[a as usize][b as usize]
    }

    pub fn get_cell(&self, &pos: &[i8; 2]) -> Cell {
        if !Self::is_pos_valid(&pos) { Cell::OutOfBounds } 
        else { Self::get_cell_from_layout(&self.layout, &pos) } 
    }

    pub fn change_pos(&mut self, &new_pos: &[i8; 2]
    ) -> Result<(), String> {
        match self.get_cell(&new_pos) {
            Cell::OutOfBounds => 
                Err(format!("New position out of bounds. Got: {:?}", new_pos)),
            Cell::Wall => 
                Err(format!("New position is wall. Got: {:?}", new_pos)),
            Cell::Player => Ok(()),
            Cell::Empty | Cell::Goal => {
                self.current_pos = new_pos; Ok(())
            }
        }
    }

    pub fn get_relative_pos(&self, d: &Dir) -> [i8; 2] {
        let mut pos = self.current_pos;

        match d {
            Dir::Down => pos[0] += 1,
            Dir::Left => pos[1] -= 1,
            Dir::Right => pos[1] += 1,
            Dir::Up => pos[0] -= 1,
        }
        pos
    }

    pub fn move_player(&mut self, &d: &Dir) -> Result<bool, String> {
        let pos: [i8; 2] = self.get_relative_pos(&d);
        match self.get_cell(&pos) {
            Cell::OutOfBounds => 
                Err(format!("Cannot move {:?}: Out of bounds", d)),
            Cell::Wall => Ok(false),
            Cell::Player => Err(format!("Unexpected player at {:?}", pos)),
            Cell::Empty => {
                self.change_pos(&pos)?;
                Ok(true)
            },
            Cell::Goal => {
                self.change_pos(&pos)?;
                self.redraw_goal = Some(pos);
                Ok(true)
            }
        }
    }

    pub fn tick(&mut self) {
        match self.player_state {
            None => return,
            Some(dir) => {
                let i_pos = self.current_pos;
                //unwrap should not panic
                if !self.move_player(&dir).or::<bool>(Ok(false)).unwrap() {
                    self.player_state = None;
                } else {
                    self.layout[i_pos[0] as usize][i_pos[1] as usize] = 
                        if self.redraw_goal == Some(i_pos) {
                            self.redraw_goal = None; Cell::Goal
                        } else {Cell::Empty};
                    self.layout
                        [self.current_pos[0] as usize]
                        [self.current_pos[1] as usize] 
                        = Cell::Player;
                }
            }
        }
    }

    pub fn is_done(&self) -> bool {
        self.player_state == None && self.current_pos == self.goal_pos
    }

    pub fn display(&self, window: &Window, title: &String) {
        window.erase();
        window.addch('\n');
        for row in &self.layout{
            window.addch(' ');
            for c in row{
                match c {
                    Cell::Wall => window.addch('#'),
                    Cell::Empty => {
                        window.attron(COLOR_PAIR(BLACK));
                        window.addch('.');
                        window.attroff(COLOR_PAIR(BLACK))
                    },
                    Cell::Player => {
                        window.attron(Attribute::Blink);
                        window.attron(COLOR_PAIR(RED));
                        window.addch('O');
                        window.attroff(COLOR_PAIR(RED));
                        window.attroff(Attribute::Blink)
                    },
                    Cell::Goal => {
                        window.attron(COLOR_PAIR(GREEN));
                        window.addch('P');
                        window.attroff(COLOR_PAIR(GREEN))
                    },
                    _ => window.addch('?'),
                };
                window.addch(' ');
            };
            // automatic wrap since window has len LENX*2+1
            // window.addch('\n');
        };
        window.draw_box(0, 0);
        window.mvprintw(0, 3, title);
        window.mv(0, 0);
        window.refresh();
    }
}

pub fn run_level<const LENY: usize, const LENX: usize>(
    root: &Window, 
    level: &mut Level<LENY, LENX>,
    title: &String
) -> Result<Menuitems, String>{
    let size = (LENY as i32 + 2, LENX as i32 * 2 + 1);
    let mut dir: Dir;
    let mut window = root.subwin(
        size.0, size.1, 0, 0)
        .or(Err("Error creating subwindow. Perhaps window too small?"))?;
    window.keypad(true);

    let resize = 
        |window: &mut Window, level: &Level<LENY, LENX>| {
            let y = (root.get_max_y() - window.get_max_y()) / 2;
            let x = (root.get_max_x() - window.get_max_x()) / 2;
            if y < 0 || x < 0 { return };
            resize_term(0,0);
            window.resize(size.0, size.1);
            match window.mvwin(y, x) {
                -1 => panic!("{:?}", (y, x)),
                _ => {
                    root.clear();
                    root.refresh();
                    level.display(window, title);
                    window.refresh();
                }
            };
        };
        
    resize(&mut window, &level); // centers level
    level.display(&window, title);
    while !level.is_done() {
        // window.addstr();
        window.nodelay(false);
        match window.getch() {
            Some(Input::Character('q')) => {
                endwin(); 
                return Ok(Menuitems::Exit);
            },
            Some(Input::KeyUp) => dir = Dir::Up,
            Some(Input::KeyDown) => dir = Dir::Down,
            Some(Input::KeyLeft) => dir = Dir::Left,
            Some(Input::KeyRight) => dir = Dir::Right,
            Some(Input::KeyResize) => {
                resize(&mut window, &level);
                continue;
            }
            _ => continue,
        };

        level.player_state = Some(dir);
        loop {
            if let None = level.player_state {break}
            window.nodelay(true);
            level.tick();
            level.display(&window, title);
            window.refresh();            
            sleep(Duration::from_millis(30));
            // prevents key spamming issue
            while let Some(input) = window.getch() {
                match input {
                    Input::Character('q') => {
                        endwin();
                        return Ok(Menuitems::Exit);
                    }
                    Input::KeyResize => {
                        resize(&mut window, &level);
                    },
                    _ => ()
                }
            }
        }
        window.nodelay(false);
        window.refresh();
    }
    Ok(Menuitems::Next)
}

#[derive(Clone, Copy, Debug)]
pub enum Menuitems {
    Next, Select, Exit
}

impl Menuitems {
    pub const fn array() -> [Menuitems; 3] {
        use Menuitems::*;
        [Next, Select, Exit]
    }
    pub fn to_str(&self) -> &str {
        match self {
            Menuitems::Next => "Next level",
            Menuitems::Select => "Select Level",
            Menuitems::Exit => "Exit"
        }
    }
    pub fn to_str_main_menu(&self) -> &str {
        match self {
            Menuitems::Next => "Start",
            item => item.to_str()
        }
    }
    pub const fn size() -> (i32, i32) {(5, 20)}
}


#[allow(non_upper_case_globals)]
pub fn menu(root: &Window) -> Result<Menuitems, &str>{
    let mut selected: usize = 0;
    const selected_max: usize = Menuitems::array().len() - 1;
    
    let resize = |window: &mut Window| {
        let size = Menuitems::size();
        let y = (root.get_max_y() - size.0) / 2;
        let x = (root.get_max_x() - size.1) / 2;
        if y < 0 || x < 0 { return };
        match window.mvwin(y, x) {
            -1 => panic!("{:?}", (y, x)),
            _ => {
                root.erase();
                root.refresh();
                window.refresh();
            }
        }
    };

    let mut window = root.subwin(
        Menuitems::size().0, 
        Menuitems::size().1,
        0, 0
    ).map_err(|_| "Cound not create window, terminal too small?")?;

    window.nodelay(false);
    resize(&mut window);
    // window.attrset(COLOR_PAIR(BLACK));
    window.refresh();
    root.refresh();
    loop {
        window.erase();
        
        for (i, item) in Menuitems::array().iter().enumerate() {
            if i == selected {
                window.attron(COLOR_PAIR(GREEN));
                window.mvaddstr(i as i32 + 1, 2, 
                    format!("> {}", item.to_str_main_menu()));
                window.attroff(COLOR_PAIR(GREEN));
            } else {
                window.attron(COLOR_PAIR(BLACK));
                window.mvaddstr(i as i32 + 1, 4, 
                    format!("{}", item.to_str_main_menu()));
            }
            window.addch('\n');
        }

        window.draw_box(0, 0);
        window.mvaddstr(0, 2, "Menu");

        window.refresh();
        match root.getch().expect("Should wait for input") {
            Input::KeyUp if selected > 0 => selected -= 1,
            Input::KeyUp => selected = selected_max,
            Input::KeyDown if selected < selected_max => selected += 1,
            Input::KeyDown => selected = 0,
            Input::Character('q') => return Ok(Menuitems::Exit),
            Input::KeyEnter | Input::Character('\n') => 
                return Ok(Menuitems::array()[selected]),
            Input::KeyResize => resize(&mut window),
            _ => continue
        }

    }


}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_level() -> Level<6, 12>{
        Level::new(
            &[0,0], 
            &[11,6], 
            [[Cell::Empty; 12]; 6]
        )
    }

    #[test]
    fn create_level() {
        assert_eq!(*empty_level().current_pos(), [0,0]);
    }

    #[test]
    fn change_pos() {
        let mut level = empty_level();
        level.change_pos(&[1, 1]).unwrap();
        assert_eq!(*level.current_pos(), [1, 1])
    }

    #[test]
    fn change_pos_illegal() {
        let mut level = empty_level();
        level.change_pos(&[12, 6])
            .expect_err("Should raise an error as out of bound");
    }
}