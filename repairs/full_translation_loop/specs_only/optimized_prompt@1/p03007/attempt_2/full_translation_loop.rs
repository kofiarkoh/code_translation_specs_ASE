use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of elements in the list
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.parse().unwrap_or(0), // Default to 0 if parsing fails
        _ => return, // Exit if there's no input
    };

    let mut numbers: Vec<i64> = Vec::with_capacity(n);

    // Read the list of integers
    for _ in 0..n {
        match lines.next() {
            Some(Ok(line)) => {
                if let Ok(number) = line.parse::<i64>() {
                    numbers.push(number);
                } else {
                    return; // Exit if parsing fails
                }
            }
            _ => return, // Exit if there's no input
        }
    }

    // Compute the absolute sum
    let absolute_sum: i64 = numbers.iter().map(|&x| x.abs()).sum();
    println!("{}", absolute_sum);

    // Determine the state of the list
    let all_non_negative = numbers.iter().all(|&x| x >= 0);
    let all_non_positive = numbers.iter().all(|&x| x <= 0);

    // Output pairs based on the state of the list
    for &num in &numbers {
        println!("{} {}", num, absolute_sum);
    }
}
