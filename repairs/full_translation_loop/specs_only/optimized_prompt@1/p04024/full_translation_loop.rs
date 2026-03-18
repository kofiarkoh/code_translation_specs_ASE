const MOD: i32 = 1_000_000_007;

fn process_grid(h: usize, w: usize, k: usize, grid: Vec<Vec<char>>) -> i32 {
    // Preconditions check
    if k == 0 {
        return 1; // Immediate return if K is zero
    }

    // Check for rows and columns filled with '#'
    let mut all_hash_row = false;
    let mut all_hash_col = false;

    // Check each row
    for row in &grid {
        if row.iter().all(|&c| c == '#') {
            all_hash_row = true;
            break;
        }
    }

    // Check each column
    for col in 0..w {
        if (0..h).all(|row| grid[row][col] == '#') {
            all_hash_col = true;
            break;
        }
    }

    // If any row or column is entirely filled with '#', return 1
    if all_hash_row || all_hash_col {
        return 1;
    }

    // Initialize result based on the grid configuration
    let mut result = 0;

    // Count the number of '#' characters in the grid
    for row in &grid {
        for &c in row {
            if c == '#' {
                result = (result + 1) % MOD; // Count '#' and keep it modulo MOD
            }
        }
    }

    // The computation logic based on the grid and K
    // Here we will compute the result based on the number of '#' characters
    // and the parameter K, as per the specification's intent.
    result = (result * k as i32) % MOD;

    result // Return the computed result
}

fn main() {
    // Example usage of the process_grid function
    let h = 3;
    let w = 4;
    let k = 2;
    let grid = vec![
        vec!['.', '#', '.', '.'],
        vec!['#', '#', '.', '#'],
        vec!['#', '.', '.', '.'],
    ];

    let result = process_grid(h, w, k, grid);
    println!("{}", result); // Output the result
}
