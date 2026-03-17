use std::io::{self, BufRead};
use std::collections::{HashSet, VecDeque};

const PUZZLE_SIZE: usize = 12; // Corrected to match the expected puzzle size
const MAX_MOVES: usize = 16;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PuzzleState {
    tiles: [i32; PUZZLE_SIZE],
}

impl PuzzleState {
    fn is_solvable(&self) -> bool {
        // Implement solvability check for the Eleven Puzzle
        // Placeholder for actual solvability logic
        true
    }

    fn get_neighbors(&self) -> Vec<PuzzleState> {
        // Generate all valid neighbor states by swapping tiles
        // Placeholder for actual neighbor generation logic
        vec![]
    }
}

fn bfs(start: PuzzleState) -> Result<usize, &'static str> {
    if !start.is_solvable() {
        return Err("NA");
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start, 0)); // (state, moves)

    while let Some((current_state, moves)) = queue.pop_front() {
        if moves > MAX_MOVES {
            return Err("NA");
        }

        // Check if the current state is the goal state
        if current_state.tiles == [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11] {
            return Ok(moves);
        }

        for neighbor in current_state.get_neighbors() {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor.clone());
                queue.push_back((neighbor, moves + 1));
            }
        }
    }

    Err("NA")
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let tiles: Vec<i32> = input.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if tiles.is_empty() || tiles[0] < 0 {
        println!("NA"); // Changed to print "NA" instead of terminating
        return;
    }

    if tiles.len() != PUZZLE_SIZE {
        println!("NA");
        return; // Invalid input size
    }

    let mut initial_state = PuzzleState { tiles: [0; PUZZLE_SIZE] };
    initial_state.tiles.copy_from_slice(&tiles);

    match bfs(initial_state) {
        Ok(moves) => println!("{}", moves),
        Err(_) => println!("NA"),
    }
}
