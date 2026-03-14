use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut graph = vec![-1; 101]; // Initialize graph with -1
    loop {
        let mut from = -1;
        let mut to = -1;
        let mut odd_num = 0;

        // Reset graph for each iteration
        for i in 1..=100 {
            graph[i] = -1;
        }

        // Read pairs of integers from stdin
        for line in stdin.lock().lines() {
            let line = line.unwrap();
            let nums: Vec<i32> = line.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            if nums.len() < 2 {
                continue; // Skip if not enough numbers
            }

            from = nums[0];
            to = nums[1];

            if from == 0 && to == 0 {
                break;
            } else {
                graph[to as usize] = if graph[to as usize] == -1 { 1 } else { graph[to as usize] + 1 };
                graph[from as usize] = if graph[from as usize] == -1 { 1 } else { graph[from as usize] + 1 };
            }
        }

        if from != 0 || to != 0 {
            break;
        }

        // Count odd occurrences
        for i in 1..=100 {
            if graph[i] == -1 {
                break;
            }
            if graph[i] % 2 == 1 {
                odd_num += 1;
            }
        }

        // Output based on the count of odd occurrences
        if odd_num == 0 || odd_num == 2 {
            println!("OK");
        } else {
            println!("NG");
        }
    }
}
