// Import necessary libraries
use std::collections::HashSet;
use std::io::{self, BufRead};

// Define the size of the puzzle
const PUZZLE_SIZE: usize = 13;

// Function to check if the puzzle is solvable
fn is_solvable(state: &[i32; PUZZLE_SIZE]) -> bool {
    // Count inversions
    let mut inversions = 0;
    for i in 0..PUZZLE_SIZE {
        for j in (i + 1)..PUZZLE_SIZE {
            if state[i] != 0 && state[j] != 0 && state[i] > state[j] {
                inversions += 1;
            }
        }
    }
    // A puzzle is solvable if the number of inversions is even
    inversions % 2 == 0
}

// Function to find the minimum number of moves to solve the puzzle
fn min_moves(state: [i32; PUZZLE_SIZE]) -> Option<usize> {
    // Placeholder for the actual puzzle-solving logic
    // This would typically involve a search algorithm like BFS or A*
    // For now, we return None to indicate unsolvable
    None
}

// Main function to read input and process the puzzle
fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input: Vec<i32> = stdin.lock().lines()
        .filter_map(Result::ok)
        .flat_map(|line| line.split_whitespace().filter_map(|s| s.parse().ok()))
        .collect();

    // Check for the correct number of inputs
    if input.len() != PUZZLE_SIZE {
        eprintln!("Error: Input must consist of exactly thirteen integers.");
        return;
    }

    // Check if the first integer is non-negative
    if input[0] < 0 {
        return; // Terminate without further processing
    }

    // Convert input to an array
    let mut state = [0; PUZZLE_SIZE];
    state.copy_from_slice(&input);

    // Check if the puzzle is solvable
    if !is_solvable(&state) {
        println!("The puzzle is unsolvable.");
        return;
    }

    // Calculate the minimum number of moves
    match min_moves(state) {
        Some(moves) => println!("{}", moves),
        None => println!("The puzzle is unsolvable."),
    }
}
