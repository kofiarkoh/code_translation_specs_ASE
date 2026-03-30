use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let dims: Vec<usize> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let (n, m) = (dims[0], dims[1]);
        if n == 0 && m == 0 {
            break; // Termination condition
        }

        let mut matrix = vec![vec![0; m]; n];
        for i in 0..n {
            let row: Vec<i32> = lines.next().unwrap().unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            matrix[i] = row;
        }

        let mut column_sums = vec![0; m];
        for j in 0..m {
            for i in 0..n {
                column_sums[j] += matrix[i][j];
            }
        }

        let mut indices: Vec<usize> = (1..=m).collect(); // 1-based indices
        indices.sort_by(|&a, &b| column_sums[b - 1].cmp(&column_sums[a - 1]));

        // Output the sorted indices
        println!("{}", indices.iter().map(|&i| i.to_string()).collect::<Vec<String>>().join(" "));
    }
}
