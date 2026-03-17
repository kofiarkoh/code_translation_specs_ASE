fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the first line containing H, W, K
    let mut first_line = String::new();
    reader.read_line(&mut first_line).unwrap();
    let mut params = first_line.split_whitespace()
        .map(|s| s.parse::<i64>().unwrap());

    let H = params.next().unwrap(); // Height of the grid
    let W = params.next().unwrap(); // Width of the grid
    let K = params.next().unwrap(); // Parameter influencing the output

    // If K is 0, output 1 immediately
    if K == 0 {
        println!("{}", 1);
        return;
    }

    // Allocate the grid
    let mut grid = vec![vec![' '; W as usize]; H as usize];

    // Read the grid from input
    for i in 0..H {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        for (j, c) in line.trim().chars().enumerate() {
            grid[i as usize][j] = c; // Populate the grid
        }
    }

    // Initialize the result variable
    let mut result: i64 = 0;

    // Process the grid to compute the result based on '#' characters
    for row in grid.iter() {
        for &cell in row.iter() {
            if cell == '#' {
                // Example computation: count the number of '#' characters
                result += 1; // This is a placeholder for actual logic
            }
        }
    }

    // Output the result modulo 10^9 + 7
    const MOD: i64 = 1_000_000_007;
    result %= MOD;
    println!("{}", result);
}
