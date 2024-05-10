use rand::Rng;
use std::collections::VecDeque;
use std::io::Write;

const GRID_SIZE: usize = 4;
const INITIAL_TILES: usize = 2;
const WINNING_TILE: u32 = 2048;

struct Game {
    grid: [[u32; GRID_SIZE]; GRID_SIZE],
    score: u32,
}

