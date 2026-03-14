use std::collections::VecDeque;
use std::io::{self, BufRead};

const TARGET_STATE: [[u8; 4]; 2] = [[1, 2, 3, 4], [5, 6, 7, 0]]; // Target state for the puzzle

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PuzzleState {
    grid: [[u8; 4]; 2],
    zero_pos: (usize, usize),
}

impl PuzzleState {
    fn new(config: &str) -> Self {
        let mut grid = [[0; 4]; 2];
        let mut zero_pos = (0, 0);
        for (i, c) in config.chars().enumerate() {
            let value = c.to_digit(10).unwrap() as u8;
            grid[i / 4][i % 4] = value;
            if value == 0 {
                zero_pos = (i / 4, i % 4);
            }
        }
        PuzzleState { grid, zero_pos }
    }

    fn is_target(&self) -> bool {
        self.grid == TARGET_STATE
    }

    fn generate_next_states(&self) -> Vec<PuzzleState> {
        let mut next_states = Vec::new();
        let (x, y) = self.zero_pos;
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)]; // Down, Up, Right, Left

        for (dx, dy) in directions.iter() {
            let new_x = (x as isize + dx) as usize;
            let new_y = (y as isize + dy) as usize;

            if new_x < 2 && new_y < 4 {
                let mut new_grid = self.grid.clone();
                new_grid[x][y] = new_grid[new_x][new_y];
                new_grid[new_x][new_y] = 0;
                next_states.push(PuzzleState {
                    grid: new_grid,
                    zero_pos: (new_x, new_y),
                });
            }
        }
        next_states
    }
}

fn bfs(start: PuzzleState) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = std::collections::HashSet::new();
    queue.push_back((start, 0)); // (state, moves)

    while let Some((current, moves)) = queue.pop_front() {
        if current.is_target() {
            return Some(moves);
        }

        for next in current.generate_next_states() {
            if !visited.contains(&next) {
                visited.insert(next.clone());
                queue.push_back((next, moves + 1));
            }
        }
    }
    None
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let puzzle_state = PuzzleState::new(&line);
        if let Some(moves) = bfs(puzzle_state) {
            println!("{}", moves);
        } else {
            println!("No solution found"); // This case should not happen per the spec
        }
    }
}
