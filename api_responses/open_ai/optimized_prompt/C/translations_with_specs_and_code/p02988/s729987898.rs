fn main() {
    use std::io::{self, BufRead};

    // Read input from stdin
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line for the number of integers
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(0),
        _ => 0,
    };

    // Check precondition: n must be >= 3
    if n < 3 {
        println!("0");
        return;
    }

    // Read the subsequent integers into a vector
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            if let Ok(num) = line.trim().parse::<i32>() {
                nums.push(num);
            }
        }
    }

    // Initialize count of sequences
    let mut count = 0;

    // Count strictly increasing or strictly decreasing sequences of three consecutive integers
    for i in 0..(n - 2) {
        if nums[i] < nums[i + 1] && nums[i + 1] < nums[i + 2] {
            count += 1;
        } else if nums[i] > nums[i + 1] && nums[i + 1] > nums[i + 2] {
            count += 1;
        }
    }

    // Output the count
    println!("{}", count);
}
