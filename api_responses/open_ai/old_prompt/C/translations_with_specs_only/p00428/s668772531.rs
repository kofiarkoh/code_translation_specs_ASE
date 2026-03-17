use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Continuously read input until termination condition (0 0) is met
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let dimensions: Vec<usize> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();

        // Termination condition
        if dimensions == vec![0, 0] {
            break;
        }

        let (n, m) = (dimensions[0], dimensions[1]);

        // Ensure n and m are positive integers
        if n == 0 || m == 0 {
            continue; // Undefined behavior as per specification; we skip this input
        }

        // Initialize the matrix and column sums
        let mut matrix = vec![vec![0; m]; n];
        let mut column_sums = vec![0; m];

        // Read the matrix data
        for i in 0..n {
            let line = reader.lines().next().expect("Failed to read line");
            let row: Vec<i32> = line
                .expect("Failed to read line")
                .split_whitespace()
                .map(|s| s.parse().expect("Invalid input"))
                .collect();

            // Store the row in the matrix and update column sums
            for j in 0..m {
                matrix[i][j] = row[j];
                column_sums[j] += row[j];
            }
        }

        // Create a vector of column indices
        let mut indices: Vec<usize> = (0..m).collect();

        // Sort indices based on column sums in descending order
        indices.sort_by(|&a, &b| column_sums[b].cmp(&column_sums[a]));

        // Output the sorted indices (1-based)
        let output: Vec<String> = indices.iter().map(|&i| (i + 1).to_string()).collect();
        println!("{}", output.join(" "));
    }
}
