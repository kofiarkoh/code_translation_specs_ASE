use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut nums: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Check if the first number is zero, which indicates termination
        if nums.is_empty() || nums[0] == 0 {
            break;
        }

        let n = nums[0] as usize; // The first number indicates the count of subsequent integers
        if n == 0 || nums.len() < n + 1 {
            continue; // Invalid sequence, skip to the next
        }

        // Process the sequence
        let mut go = nums[1..=n].to_vec(); // Slice to get the relevant integers
        for i in (1..n).step_by(2) {
            if go[i - 1] != go[i] {
                for j in (0..=i - 1).rev() {
                    if go[j] != go[i] {
                        go[j] = go[i];
                    }
                }
            }
        }

        // Count the zeros in the modified sequence
        let cnt = go.iter().filter(|&&x| x == 0).count();
        println!("{}", cnt);
    }
}
