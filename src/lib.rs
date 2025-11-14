use std::{thread::sleep, time::Duration, usize};

use pancurses::{endwin, Input, Window};

// pos: position
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Empty, Wall, OutOfBounds, Player, Goal
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dir {
    Up, Down, Left, Right
}

#[derive(Debug)]
#[allow(unused)]
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

    fn new(&start_pos: &[i8; 2], &goal_pos: &[i8; 2], layout: [[Cell; LENX]; LENY]) -> Level<LENY, LENX>{
        // take ownership of layout(procesed)
        Level {start_pos, current_pos: start_pos, goal_pos, layout, player_state: None, redraw_goal: None, size: [LENY, LENX]}
    }

    pub fn build(
        &start_pos: &[i8; 2], 
        &goal_pos: &[i8; 2], 
        layout: &[[Cell; LENX]; LENY],
    ) -> Result<Level<LENY,LENX>, String> {
        let size: [i8; 2] = [LENY.try_into().map_err(|_| "Len too big")?, LENX.try_into().map_err(|_| "Len too big")?];
        Self::check_pos_valid_from_size(&start_pos, &size)?;
        Self::check_pos_valid_from_size(&goal_pos, &size)?;
        if Self::get_cell_from_layout(&layout, &start_pos) == Cell::Wall { return Err("Invalid start pos: start pos must be empty".to_string())}
        if Self::get_cell_from_layout(&layout, &goal_pos) == Cell::Wall { return Err("Invalid end pos: end pos must be empty".to_string())}
        let mut processed = layout.clone();
        processed[start_pos[0] as usize][start_pos[1] as usize] = Cell::Player;
        processed[goal_pos[0] as usize][goal_pos[1] as usize] = Cell::Goal;
        // not complete checks, but checking if solvable requires solving it
        // will at least prevent panicking due to invalid pos
        Ok(Level::new(&start_pos, &goal_pos, processed))
    }

    pub fn build_from_tuple(
        (&start_pos, &goal_pos, layout, ): 
        (&[i8; 2], &[i8; 2], &[[Cell; LENX]; LENY])) -> Result<Level<LENY, LENX>, String> {
            Self::build(&start_pos, &goal_pos, &layout)
        }

    pub fn is_pos_valid(&pos: &[i8; 2]) -> bool {
        let [y, x] = pos;
        (..LENY).contains(&(y as usize)) && (..LENX).contains(&(x as usize))
    }

    pub fn check_pos_valid_from_size(&pos: &[i8; 2], &size: &[i8; 2]) -> Result<(), String> {
        let ([may, max], [y, x]) = (size, pos);
        if (..may).contains(&y) && (..max).contains(&x) {Ok(())}
        else {Err(format!("Invalid position: {:?}", pos))}
    }

    fn get_cell_from_layout(layout: &[[Cell; LENX]; LENY], &pos: &[i8; 2]) -> Cell {
        let [a, b] = pos;
        layout[a as usize][b as usize]
    }

    pub fn get_cell(&self, &pos: &[i8; 2]) -> Cell {
        if !Self::is_pos_valid(&pos) { Cell::OutOfBounds } 
        else { Self::get_cell_from_layout(&self.layout, &pos) } 
    }

    pub fn change_pos(&mut self, &new_pos: &[i8; 2]) -> Result<(), String> {
        match self.get_cell(&new_pos) {
            Cell::OutOfBounds => Err(format!("New position out of bounds. Got: {:?}", new_pos)),
            Cell::Wall => Err(format!("New position is wall. Got: {:?}", new_pos)),
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
            Cell::OutOfBounds => Err(format!("Cannot move {:?}: Out of bounds", d)),
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
                if !self.move_player(&dir).or::<bool>(Ok(false)).unwrap() {
                    self.player_state = None;
                } else {
                    self.layout[i_pos[0] as usize][i_pos[1] as usize] = 
                        if self.redraw_goal == Some(i_pos) {self.redraw_goal = None; Cell::Goal} else {Cell::Empty};
                    self.layout[self.current_pos[0] as usize][self.current_pos[1] as usize] = Cell::Player;
                }
            }
        }
    }

    pub fn is_done(&self) -> bool {
        self.player_state == None && self.current_pos == self.goal_pos
    }

    pub fn to_string(&self) -> String{
        let mut result_str: String = String::new();
        for row in &self.layout{
            for c in row{
                result_str += match c {
                    Cell::Wall => "#",
                    Cell::Empty => ".",
                    Cell::Player => "O",
                    Cell::Goal => "F",
                    _ => "?",
                };
                result_str += " "
            };
            result_str += "\n"
        }
        result_str
    }
}

pub fn run_level<const LENY: usize, const LENX: usize>(window: &Window, level: &mut Level<LENY, LENX>) -> Result<(), String>{
    let mut dir: Option<Dir>;
    window.clear();
    window.addstr(level.to_string());
    window.refresh();
    while !level.is_done() {
        // window.addstr();
        match window.getch() {
            Some(Input::Character('q')) => {endwin(); return Err("User hit <q>".to_string());},
            Some(Input::KeyUp) => dir = Some(Dir::Up),
            Some(Input::KeyDown) => dir = Some(Dir::Down),
            Some(Input::KeyLeft) => dir = Some(Dir::Left),
            Some(Input::KeyRight) => dir = Some(Dir::Right),
            _ => continue,
        };
        level.player_state = dir;
        loop {
            if let None = level.player_state {break}
            level.tick();
            window.clear();
            window.addstr(level.to_string());
            window.refresh();
            sleep(Duration::from_millis(50));
        }
        window.refresh();
    }
    Ok(())
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
        level.change_pos(&[12, 6]).expect_err("Should raise an error as out of bound");
    }
}