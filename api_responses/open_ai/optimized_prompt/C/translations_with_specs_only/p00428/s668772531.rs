use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let dims: Vec<usize> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // Check for termination condition (0, 0)
        if dims.len() != 2 || dims[0] == 0 && dims[1] == 0 {
            break;
        }

        let (n, m) = (dims[0], dims[1]);
        let mut matrix = vec![vec![0; m]; n];
        let mut column_sums = vec![0; m];

        // Read the matrix
        for i in 0..n {
            let line = reader.lines().next().unwrap().unwrap();
            let row: Vec<i32> = line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            for j in 0..m {
                matrix[i][j] = row[j];
                column_sums[j] += row[j]; // Maintain column sums
            }
        }

        // Create a vector of (index, sum) pairs
        let mut indexed_sums: Vec<(usize, i32)> = column_sums.iter()
            .enumerate()
            .map(|(index, &sum)| (index + 1, sum)) // Convert to 1-based index
            .collect();

        // Sort by sum in descending order
        indexed_sums.sort_by(|a, b| b.1.cmp(&a.1));

        // Output the indices in the required order
        let output: Vec<String> = indexed_sums.iter()
            .map(|&(index, _)| index.to_string())
            .collect();
        println!("{}", output.join(" "));
    }
}
