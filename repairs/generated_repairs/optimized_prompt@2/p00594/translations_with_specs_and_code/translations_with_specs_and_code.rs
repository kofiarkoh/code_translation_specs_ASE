use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut nums: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // The first number indicates the count of subsequent integers
        let n = match nums.get(0) {
            Some(&num) if num > 0 => num as usize,
            _ => continue, // Skip if the first number is not valid
        };

        // If the first number is zero, we stop processing
        if n == 0 {
            break;
        }

        // Read the integers into the array
        nums.truncate(n + 1); // Ensure we only consider the first n integers
        for i in 1..=n {
            let num: i32 = match reader.lines().next() {
                Some(Ok(line)) => line.trim().parse().unwrap(),
                _ => break,
            };
            nums[i] = num;
        }

        // Sort the array
        nums[1..=n].sort_unstable();

        let mut max_length = 0;
        let mut most_frequent = 0;

        let mut i = 1;
        while i <= n {
            let mut j = i + 1;
            while j <= n && nums[j] == nums[i] {
                j += 1;
            }
            let length = j - i;
            if length > max_length {
                max_length = length;
                most_frequent = nums[i];
            }
            i = j; // Move to the next distinct element
        }

        // Check the condition for output
        if max_length > n / 2 {
            println!("{}", most_frequent);
        } else {
            println!("NO COLOR");
        }
    }
}
