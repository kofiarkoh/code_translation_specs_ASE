use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut dimensions = line.split_whitespace();
        let n: usize = dimensions.next().unwrap().parse().unwrap();
        let m: usize = dimensions.next().unwrap().parse().unwrap();

        // Termination condition
        if n == 0 && m == 0 {
            break;
        }

        // Read the matrix
        let mut matrix = vec![vec![0; m]; n];
        for i in 0..n {
            let row: Vec<i32> = reader
                .lines()
                .take(1)
                .map(|line| line.unwrap())
                .flat_map(|line| line.split_whitespace().map(|s| s.parse().unwrap()))
                .collect();
            matrix[i].copy_from_slice(&row);
        }

        // Calculate column sums
        let mut column_sums = vec![0; m];
        for i in 0..n {
            for j in 0..m {
                column_sums[j] += matrix[i][j];
            }
        }

        // Sort column indices by their sums
        let mut indices: Vec<usize> = (0..m).collect();
        indices.sort_by(|&a, &b| column_sums[b].cmp(&column_sums[a]));

        // Output the sorted indices (1-based)
        for (i, &index) in indices.iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{}", index + 1);
        }
        println!();
    }
}
