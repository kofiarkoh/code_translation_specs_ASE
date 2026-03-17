use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read integers from input
    let mut numbers: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let n: i32 = line.trim().parse().expect("Invalid integer input");
        
        // Terminate if n is 0
        if n == 0 {
            break;
        }

        // Read the next n integers
        for _ in 0..n {
            let num: i32 = reader.lines().next().expect("Failed to read line").expect("Invalid integer input").trim().parse().expect("Invalid integer input");
            numbers.push(num);
        }

        // Process the numbers to find the dominant integer
        let result = find_dominant_integer(&numbers);
        println!("{}", result);
    }
}

// Function to find the dominant integer
fn find_dominant_integer(numbers: &[i32]) -> String {
    let n = numbers.len();
    if n == 0 {
        return "NO COLOR".to_string(); // Edge case: no numbers
    }

    // Count occurrences of each integer
    let mut counts = HashMap::new();
    for &num in numbers {
        *counts.entry(num).or_insert(0) += 1;
    }

    // Find the integer with the maximum count
    let mut dominant_color = None;
    let mut max_count = 0;

    for (&num, &count) in &counts {
        if count > max_count {
            max_count = count;
            dominant_color = Some(num);
        }
    }

    // Check if the max count exceeds half of n
    if max_count > n / 2 {
        dominant_color.map_or("NO COLOR".to_string(), |color| color.to_string())
    } else {
        "NO COLOR".to_string()
    }
}
