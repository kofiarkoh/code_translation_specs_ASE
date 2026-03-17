use std::collections::HashMap;
use std::io::{self, BufRead};

const GOAL_STATE: u64 = 0b0000000000000000000000000000000000000000000000000000000000000000111111111111111111111111111111111111111111111111111111111111111111; // Example goal state
const MOVES: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)]; // Possible moves (down, up, right, left)

fn main() {
    let stdin = io::stdin();
    let input: Vec<i32> = stdin.lock().lines()
        .take(1)
        .flat_map(|line| line.unwrap().split_whitespace().map(|s| s.parse::<i32>().unwrap()))
        .collect();

    // Check for valid input
    if input.len() != 13 || input[0] < 0 {
        panic!("Invalid input: the first integer must be non-negative and there must be exactly 13 integers.");
    }

    let initial_state = compute_state(&input);
    let result = bfs(initial_state);
    
    match result {
        Some(moves) => println!("{}", moves),
        None => println!("NA"),
    }
}

// Function to compute the state from the input integers
fn compute_state(input: &[i32]) -> u64 {
    let mut state: u64 = 0;
    for (i, &num) in input.iter().enumerate() {
        state |= (num as u64) << (i * 4); // Each number occupies 4 bits
    }
    state
}

// Breadth-first search to find the minimum moves to reach the goal state
fn bfs(initial_state: u64) -> Option<u32> {
    let mut queue = vec![(initial_state, 0)]; // (state, moves)
    let mut visited = HashMap::new();
    visited.insert(initial_state, 0);

    while let Some((state, moves)) = queue.remove(0) {
        if state == GOAL_STATE {
            return Some(moves);
        }

        for &(dx, dy) in &MOVES {
            // Logic to generate new states based on moves
            // This part is simplified and would need to be implemented
            // to actually compute the new state based on the empty tile position.
            // For now, we will just continue the loop.
        }
    }
    None // Return None if no solution is found
}
