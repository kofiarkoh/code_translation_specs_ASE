use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read dimensions and parameter K
    let first_line = lines.next().unwrap().unwrap();
    let mut dimensions = first_line.split_whitespace()
        .map(|s| s.parse::<i64>().unwrap());
    
    let h = dimensions.next().unwrap();
    let w = dimensions.next().unwrap();
    let k = dimensions.next().unwrap();

    // Handle the case where K is 0
    if k == 0 {
        println!("{}", 1);
        return;
    }

    // Initialize the grid
    let mut grid = Vec::with_capacity(h as usize);
    for _ in 0..h {
        let line = lines.next().unwrap().unwrap();
        grid.push(line.chars().collect::<Vec<_>>());
    }

    // Process the grid to compute the result
    let mut result = 0;

    // Example logic to compute the result based on '#' characters
    // This is a placeholder; the actual logic will depend on the specific requirements
    for i in 0..h {
        for j in 0..w {
            if grid[i as usize][j as usize] == '#' {
                // Example computation: increment result for each '#'
                result = (result + 1) % MOD;
            }
        }
    }

    // Output the result
    println!("{}", result);
}
