use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut graph = vec![-1; 101]; // Initialize degrees to -1 for vertices 1 to 100

    loop {
        let mut odd_count = 0;
        // Reset graph degrees for each new input set
        for i in 1..=100 {
            graph[i] = -1;
        }

        // Read pairs of integers from standard input
        for line in stdin.lock().lines() {
            let line = line.expect("Failed to read line");
            let nums: Vec<i32> = line.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            if nums.len() != 2 {
                continue; // Skip invalid lines
            }

            let from = nums[0];
            let to = nums[1];

            if from == 0 && to == 0 {
                break; // End of input
            } else {
                // Update the degree of the vertices
                graph[from as usize] = if graph[from as usize] == -1 { 1 } else { graph[from as usize] + 1 };
                graph[to as usize] = if graph[to as usize] == -1 { 1 } else { graph[to as usize] + 1 };
            }
        }

        // Check for odd degree vertices
        for i in 1..=100 {
            if graph[i] == -1 {
                break; // Stop checking if we hit an uninitialized vertex
            }
            if graph[i] % 2 == 1 {
                odd_count += 1;
            }
        }

        // Output result based on the number of odd degree vertices
        if odd_count == 0 || odd_count == 2 {
            println!("OK");
        } else {
            println!("NG");
        }
    }
}
