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

impl Game {
    fn new() -> Game {
        let mut game = Game {
            grid: [[0; GRID_SIZE]; GRID_SIZE],
            score: 0,
        };
        game.add_tile();
        game.add_tile();
        game
    }

    fn add_tile(&mut self) {
        let mut rng = rand::thread_rng();
        let mut row = rng.gen_range(0..GRID_SIZE);
        let mut col = rng.gen_range(0..GRID_SIZE);
        while self.grid[row][col]!= 0 {
            row = rng.gen_range(0..GRID_SIZE);
            col = rng.gen_range(0..GRID_SIZE);
        }
        self.grid[row][col] = 2;
    }

    fn move_tiles(&mut self, direction: Direction) {
        let mut moved = false;
        match direction {
            Direction::Up => {
                for col in 0..GRID_SIZE {
                    let mut stack = VecDeque::new();
                    for row in 0..GRID_SIZE {
                        if self.grid[row][col]!= 0 {
                            stack.push_back(self.grid[row][col]);
                            self.grid[row][col] = 0;
                        }
                    }
                    let mut new_row = 0;
                    while let Some(tile) = stack.pop_front() {
                        if new_row > 0 && self.grid[new_row - 1][col] == tile {
                            self.grid[new_row - 1][col] *= 2;
                            self.score += tile;
                        } else {
                            self.grid[new_row][col] = tile;
                            new_row += 1;
                        }
                    }
                    moved = true;
                }
            }
            Direction::Down => {
                for col in 0..GRID_SIZE {
                    let mut stack = VecDeque::new();
                    for row in (0..GRID_SIZE).rev() {
                        if self.grid[row][col]!= 0 {
                            stack.push_back(self.grid[row][col]);
                            self.grid[row][col] = 0;
                        }
                    }
                    let mut new_row = GRID_SIZE - 1;
                    while let Some(tile) = stack.pop_front() {
                        if new_row < GRID_SIZE - 1 && self.grid[new_row + 1][col] == tile {
                            self.grid[new_row + 1][col] *= 2;
                            self.score += tile;
                        } else {
                            self.grid[new_row][col] = tile;
                            new_row -= 1;
                        }
                    }
                    moved = true;
                }
            }
            Direction::Left => {
                for row in 0..GRID_SIZE {
                    let mut stack = VecDeque::new();
                    for col in 0..GRID_SIZE {
                        if self.grid[row][col]!= 0 {
                            stack.push_back(self.grid[row][col]);
                            self.grid[row][col] = 0;
                        }
                    }
                    let mut new_col = 0;
                    while let Some(tile) = stack.pop_front() {
                        if new_col > 0 && self.grid[row][new_col - 1] == tile {
                            self.grid[row][new_col - 1] *= 2;
                            self.score += tile;
                        } else {
                            self.grid[row][new_col] = tile;
                            new_col += 1;
                        }
                    }
                    moved = true;
                }
            }
            Direction::Right => {
                for row in 0..GRID_SIZE {
                    let mut stack = VecDeque::new();
                    for col in (0..GRID_SIZE).rev() {
                        if self.grid[row][col]!= 0 {
                            stack.push_back(self.grid[row][col]);
                            self.grid[row][col] = 0;
                        }
                    }
                    let mut new_col = GRID_SIZE - 1;
                    while let Some(tile) = stack.pop_front() {
                        if new_col < GRID_SIZE - 1 && self.grid[row][new_col + 1] == tile {
                            self.grid[row][new_col + 1] *= 2;
                            self.score += tile;
                        } else {
                            self.grid[row][new_col] = tile;
                            new_col -= 1;
                        }
                    }
                    moved = true;
                }
            }
        }
        if moved {
            self.add_tile();
        }
    }

    fn is_game_over(&self) -> bool {
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                if self.grid[row][col] == WINNING_TILE {
                    return true;
                }
                if self.grid[row][col] == 0 {
                    return false;
                }
                if col < GRID_SIZE - 1 && self.grid[row][col] == self.grid[row][col + 1] {
                    return false;
                }
                if row < GRID_SIZE - 1 && self.grid[row][col] == self.grid[row + 1][col] {
                    return false;
                }
            }
        }
        true
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let mut game = Game::new();
    loop {
        println!("Score: {}", game.score);
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                print!("{} ", game.grid[row][col]);
            }
            println!();
        }
        print!("Enter direction (w/a/s/d): ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let direction = match input.trim() {
            "w" => Direction::Up,
            "a" => Direction::Left,
            "s" => Direction::Down,
            "d" => Direction::Right,
            _ => continue,
        };
        game.move_tiles(direction);
        if game.is_game_over() {
            println!("Game over! Final score: {}", game.score);
            break;
        }
    }
}