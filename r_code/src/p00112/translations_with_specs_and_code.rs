use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut nums: Vec<i64> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if nums.is_empty() {
            continue;
        }

        let n = nums[0];
        if n == 0 {
            break; // Termination condition
        }

        // Ensure we have the correct number of time durations
        if nums.len() != (n as usize + 1) {
            continue; // Invalid input, skip this series
        }

        let mut time = nums[1..].to_vec(); // Extract time durations
        let mut wtime = vec![0; n as usize]; // Initialize waiting times
        let mut sum = 0;

        // Sort the time durations using a simple selection sort
        for j in 0..(n - 1) {
            let mut min = j;
            for k in (j + 1)..n {
                if time[min as usize] > time[k as usize] {
                    min = k;
                }
            }
            if min != j {
                time.swap(min as usize, j as usize);
            }
        }

        // Compute the weighted sum of waiting times
        for j in 1..n {
            wtime[j as usize] = wtime[(j - 1) as usize] + time[(j - 1) as usize];
            sum += wtime[j as usize];
        }

        // Output the result for the current series
        println!("{}", sum);
    }
}
