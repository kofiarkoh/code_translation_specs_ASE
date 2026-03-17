use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut nums: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // The first number indicates the length of the sequence
        let n = nums.remove(0);
        if n == 0 {
            break; // Termination condition
        }

        // Ensure we only process the expected number of integers
        if nums.len() != n as usize {
            continue; // Invalid input handling (not specified, but safe to skip)
        }

        // Modify the sequence according to the specified logic
        for i in (1..n).step_by(2) {
            if nums[i as usize - 1] != nums[i as usize] {
                for j in (0..=i).rev() {
                    if nums[j as usize] != nums[i as usize] {
                        nums[j as usize] = nums[i as usize];
                    }
                }
            }
        }

        // Count the number of zeros in the modified sequence
        let zero_count = nums.iter().filter(|&&x| x == 0).count();
        println!("{}", zero_count);
    }
}
