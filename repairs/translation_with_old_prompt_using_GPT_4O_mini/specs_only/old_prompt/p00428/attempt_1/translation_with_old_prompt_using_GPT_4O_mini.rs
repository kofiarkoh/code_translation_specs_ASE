use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();

    loop {
        let mut dimensions = String::new();
        input.read_line(&mut dimensions).unwrap();
        let dimensions: Vec<usize> = dimensions
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let (n, m) = (dimensions[0], dimensions[1]);

        // Termination condition
        if n == 0 && m == 0 {
            break;
        }

        // Read the matrix
        let mut matrix = vec![vec![0; m]; n];
        for i in 0..n {
            let mut row = String::new();
            input.read_line(&mut row).unwrap();
            matrix[i] = row
                .trim()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
        }

        // Compute column sums
        let mut column_sums = vec![0; m];
        for j in 0..m {
            for i in 0..n {
                column_sums[j] += matrix[i][j];
            }
        }

        // Create a vector of indices and sort by column sums
        let mut indices: Vec<usize> = (0..m).collect();
        indices.sort_by(|&a, &b| column_sums[b].cmp(&column_sums[a]));

        // Output the sorted indices (1-based)
        let output: Vec<String> = indices.iter().map(|&i| (i + 1).to_string()).collect();
        println!("{}", output.join(" "));
    }
}
