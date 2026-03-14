use std::io::{self, BufRead};

fn main() {
    // Initialize an array to hold the degree of each vertex (1 to 100)
    // Using 101 to accommodate 1-based indexing, where index 0 is unused.
    let mut degree: [i32; 101] = [-1; 101];

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Check for the termination condition (pair of zeros)
        if nums.len() == 2 && nums[0] == 0 && nums[1] == 0 {
            break;
        }

        // Ensure we have exactly two integers
        if nums.len() != 2 {
            continue; // Skip invalid input
        }

        let u = nums[0];
        let v = nums[1];

        // Update the degree of the vertices
        if u >= 1 && u <= 100 {
            degree[u as usize] += 1;
        }
        if v >= 1 && v <= 100 {
            degree[v as usize] += 1;
        }
    }

    // Check the degree of vertices to determine if the graph has an Eulerian path or circuit
    let mut odd_count = 0;

    for &deg in &degree[1..] { // Skip index 0
        if deg != -1 { // Only consider initialized vertices
            if deg % 2 != 0 {
                odd_count += 1;
            }
        }
    }

    // An Eulerian circuit exists if all vertices have even degree
    // An Eulerian path exists if exactly two vertices have odd degree
    if odd_count == 0 || odd_count == 2 {
        println!("OK");
    } else {
        println!("NG");
    }
}
