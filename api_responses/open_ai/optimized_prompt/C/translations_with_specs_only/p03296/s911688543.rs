fn main() {
    use std::io::{self, BufRead};

    // Read the number of integers N from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line as N
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(0),
        _ => 0,
    };

    // Preconditions: N must be a positive integer
    if n < 1 {
        // Undefined behavior as per specification; we can just return
        return;
    }

    // Vector to store the integers
    let mut numbers = Vec::with_capacity(n);

    // Read the next N integers
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            if let Ok(num) = line.trim().parse::<i32>() {
                numbers.push(num);
            }
            // Note: The specification does not handle invalid input formats or out-of-range values
        }
    }

    // Count pairs of consecutive equal integers
    let mut count = 0;
    for i in 1..numbers.len() {
        if numbers[i] == numbers[i - 1] {
            count += 1; // Increment count if consecutive integers are equal
        }
    }

    // Output the count of pairs
    println!("{}", count);
}
