use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Read the number of nodes in each partition
    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Handle the case where N is zero
    if n == 0 {
        println!("yes");
        return;
    }

    // Read the grid of connections
    let mut grid = vec![vec![0; n]; n];
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        grid[i] = line.split_whitespace()
                      .map(|x| x.parse::<i32>().unwrap())
                      .collect();
    }

    // Prepare the match and seen arrays
    let mut match_arr = vec![-1; n]; // -1 indicates no match
    let mut seen = vec![false; n];

    // Function to perform DFS for finding a match
    fn bpm(u: usize, grid: &Vec<Vec<i32>>, match_arr: &mut Vec<i32>, seen: &mut Vec<bool>, n: usize) -> bool {
        for v in 0..n {
            if grid[u][v] == 1 && !seen[v] {
                seen[v] = true; // Mark v as seen
                // If v is not matched or previously matched vertex has an alternate match
                if match_arr[v] == -1 || bpm(match_arr[v] as usize, grid, match_arr, seen, n) {
                    match_arr[v] = u as i32; // Match u with v
                    return true;
                }
            }
        }
        false
    }

    // Count of matches
    let mut result = 0;
    for u in 0..n {
        // Reset seen array for each node
        seen.fill(false);
        if bpm(u, &grid, &mut match_arr, &mut seen, n) {
            result += 1;
        }
    }

    // Output results
    if result == n {
        println!("yes");
        for v in 0..n {
            println!("{} {}", match_arr[v] + 1, v + 1); // Output matched pairs (1-indexed)
        }
    } else {
        println!("no");
    }
}
