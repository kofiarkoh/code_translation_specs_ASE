// Import necessary libraries
use std::io::{self, BufRead};

// Define a function to compute the cumulative score based on the graph structure
fn compute_cumulative_score(values: &[i32], parents: &[i32]) -> i64 {
    let n = values.len();
    let mut score: i64 = 0;

    // Create a vector to track visited nodes
    let mut visited = vec![false; n];

    // Define a recursive function to process each node
    fn process_node(node: usize, values: &[i32], parents: &[i32], visited: &mut Vec<bool>, score: &mut i64) {
        // If the node has already been visited, return to avoid cycles
        if visited[node] {
            return;
        }
        visited[node] = true; // Mark the node as visited

        // Add the value of the current node to the score
        *score += values[node] as i64;

        // Process the parent node if it exists
        let parent = parents[node];
        if parent != -1 {
            let parent_index = parent as usize;
            if parent_index < values.len() { // Check if parent_index is within bounds
                process_node(parent_index, values, parents, visited, score);
            }
        }
    }

    // Process each node in the graph
    for i in 0..n {
        if !visited[i] {
            process_node(i, values, parents, &mut visited, &mut score);
        }
    }

    score // Return the computed cumulative score
}

// Main function to read input and output the result
fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the number of nodes
    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Read the values associated with each node
    let values: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read the parent-child relationships
    let parents: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Ensure the parents vector has the same length as values
    if parents.len() != n {
        panic!("The length of parents must match the number of nodes.");
    }

    // Compute the cumulative score
    let cumulative_score = compute_cumulative_score(&values, &parents);

    // Output the result
    println!("{}", cumulative_score);
}
