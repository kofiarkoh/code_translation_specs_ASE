use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut dimensions = line.split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect::<Vec<_>>();

        if dimensions.len() != 2 {
            continue; // Skip invalid input
        }

        let (n, m) = (dimensions[0], dimensions[1]);
        if n == 0 && m == 0 {
            break; // Termination condition
        }

        let mut matrix = vec![vec![0; m]; n];
        for i in 0..n {
            let row: Vec<i32> = reader.lines()
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();
            matrix[i].copy_from_slice(&row);
        }

        let mut column_sums = vec![0; m];
        for i in 0..n {
            for j in 0..m {
                column_sums[j] += matrix[i][j];
            }
        }

        let mut maxed = vec![false; m];
        let mut output = Vec::new();

        for _ in 0..m {
            let mut max_index = None;
            for j in 0..m {
                if !maxed[j] {
                    if max_index.is_none() || column_sums[max_index.unwrap()] < column_sums[j] {
                        max_index = Some(j);
                    }
                }
            }
            if let Some(index) = max_index {
                output.push((index + 1).to_string()); // Convert to 1-based index
                maxed[index] = true;
            }
        }

        println!("{}", output.join(" "));
    }
}
