#![allow(unused)]

/// original file for levels, currently unused and shifted to json files under ./levels

use sliding_puzzle::Cell;

const E: Cell = Cell::Empty;
const W: Cell = Cell::Wall;

const LEVEL1_LAYOUT: [[Cell; 12]; 6] = [
    [E, E, E, E, E, W, E, E, E, E, W, E],
    [E, E, W, E, E, E, W, E, W, E, E, W],
    [E, E, W, E, W, W, E, E, E, E, W, E],
    [W, E, E, E, W, E, E, E, E, E, E, E],
    [E, E, E, E, E, E, E, E, E, E, E, E],
    [E, E, E, E, E, W, E, E, E, W, E, E]
];

pub const LEVEL1: (&[usize; 2], &[usize; 2], &[[Cell; 12]; 6], &str) = 
    (&[0, 0], &[4, 4], &LEVEL1_LAYOUT, "Level 1");

const LEVEL_2_LAYOUT: [[Cell; 20]; 10] = [
    [E, W, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E], 
    [E, E, E, E, E, E, E, E, W, E, E, E, E, E, E, W, E, E, E, E], 
    [W, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E], 
    [E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, W, E, E, E, E], 
    [W, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E], 
    [E, E, E, E, W, E, E, E, E, E, E, E, E, E, E, E, W, E, E, E], 
    [E, E, W, E, E, E, E, W, E, E, E, E, E, E, E, E, E, E, E, E], 
    [E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, W, E, E, E], 
    [E, E, E, W, E, W, E, E, E, E, E, E, E, E, E, E, E, E, E, E], 
    [E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E], 
];

pub const LEVEL2: (&[usize; 2], &[usize; 2], &[[Cell; 20]; 10], &str) = 
    (&[0, 0], &[1, 1], &LEVEL_2_LAYOUT, "Level 2");

const LEVEL_3_LAYOUT: [[Cell; 20]; 10] = [
    [E, E, E, E, E, E, E, W, E, E, E, E, E, E, E, E, E, E, E, E],
    [W, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E],
    [E, E, E, E, E, E, E, E, E, E, E, E, E, W, E, E, E, E, E, E],
    [E, E, E, W, E, E, E, E, E, E, E, E, E, E, E, E, E, W, E, E],
    [E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E],
    [E, E, E, E, E, E, E, E, E, E, E, W, E, E, W, E, E, E, E, E],
    [E, E, E, E, W, E, E, E, E, E, E, E, E, E, W, E, E, E, E, E],
    [E, E, E, E, E, E, W, E, E, E, E, E, E, E, E, E, E, E, E, E],
    [E, E, W, E, E, E, E, E, E, W, E, E, E, E, E, E, E, E, E, E],
    [W, E, E, E, E, E, E, E, E, E, E, E, E, W, E, E, E, E, E, W],
];

pub const LEVEL3: (&[usize; 2], &[usize; 2], &[[Cell; 20]; 10], &str) = 
    (&[0, 0], &[1, 1], &LEVEL_3_LAYOUT, "Level 3");
