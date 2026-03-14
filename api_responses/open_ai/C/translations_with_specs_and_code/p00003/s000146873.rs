use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of sets
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input");

    for _ in 0..n {
        // Read each set of three integers
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().expect("Invalid integer"))
            .collect();

        // Ensure we have exactly three integers
        if nums.len() != 3 {
            continue; // This should not happen according to the spec
        }

        // Sort the numbers to identify the largest
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        // Check the Pythagorean theorem
        if sorted_nums[0] * sorted_nums[0] + sorted_nums[1] * sorted_nums[1] == sorted_nums[2] * sorted_nums[2] {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
