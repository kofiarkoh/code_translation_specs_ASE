use std::collections::{VecDeque, HashSet};
use std::io::{self, BufRead};

const PUZZLE_SIZE: usize = 13;
const MAX_MOVES: usize = 16;

#[derive(Debug, Clone, PartialEq, Eq, Hash)] // Added Hash trait
struct Puzzle {
    tiles: [i32; PUZZLE_SIZE],
}

impl Puzzle {
    // Check if the puzzle is solvable
    fn is_solvable(&self) -> bool {
        // Placeholder for solvability check logic
        // This should implement the actual logic to determine if the puzzle can be solved
        true
    }

    // Generate possible moves from the current state
    fn generate_moves(&self) -> Vec<Puzzle> {
        let mut moves = Vec::new(); // Removed mutability warning
        // Placeholder for generating valid moves based on the current state
        // This should implement the logic to swap tiles and create new Puzzle states
        moves
    }

    // Check if the current state is the goal state
    fn is_goal(&self) -> bool {
        // Placeholder for goal state check logic
        // This should implement the actual logic to check if the puzzle is solved
        false
    }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let numbers: Vec<i32> = input.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Check for termination condition
    if numbers.is_empty() || numbers[0] < 0 {
        return;
    }

    // Ensure we have exactly 13 integers
    if numbers.len() != PUZZLE_SIZE {
        println!("NA");
        return;
    }

    let initial_puzzle = Puzzle { tiles: [
        numbers[0], numbers[1], numbers[2], 
        numbers[3], numbers[4], numbers[5], 
        numbers[6], numbers[7], numbers[8], 
        numbers[9], numbers[10], numbers[11], 
        numbers[12]
    ]};

    // Check if the puzzle is solvable
    if !initial_puzzle.is_solvable() {
        println!("NA");
        return;
    }

    // BFS to find the minimum moves
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((initial_puzzle.clone(), 0));
    visited.insert(initial_puzzle.clone());

    while let Some((current, moves)) = queue.pop_front() {
        if current.is_goal() {
            println!("{}", moves);
            return;
        }

        if moves < MAX_MOVES {
            for next in current.generate_moves() {
                if !visited.contains(&next) {
                    visited.insert(next.clone());
                    queue.push_back((next, moves + 1));
                }
            }
        }
    }

    println!("NA");
}
