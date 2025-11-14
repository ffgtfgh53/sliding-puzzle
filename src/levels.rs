use crate::Cell;

const E: Cell = Cell::Empty;
const W: Cell = Cell::Wall;

const LEVEL1_LAYOUT: [[Cell; 12]; 6] = [
    [E, E, E, E, E, W, E, E, E, E, W, E],
    [E, E, W, E, E, E, W, E, W, E, E, W],
    [E, E, W, E, W, W, E, E, E, E, W, E],
    [W, E, E, E, W, E, E, E, E, E, E, E],
    [E, E, E, E, E, E, E, E, E, E, E, E],
    [E, E, E, E, E, W, E, E, E, E, E, E]
];

pub const LEVEL1: (&[i8; 2], &[i8; 2], &[[Cell; 12]; 6]) = (&[0, 0], &[4, 4], &LEVEL1_LAYOUT);


