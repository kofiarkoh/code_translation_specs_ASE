fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the first line to get the number of elements
    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Preconditions: n must be non-negative
    if n == 0 {
        println!("YES"); // If there are no elements, we can consider it valid
        return;
    }

    // Read the sequence of integers
    let mut sequence: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let value: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        sequence.push(value);
    }

    // Calculate the sum of the sequence
    let sum: i32 = sequence.iter().sum();

    // Check if the sum can be evenly distributed
    if sum % n as i32 != 0 {
        println!("NO");
        return;
    }

    // Calculate the average
    let average = sum / n as i32;

    // Check if the adjustments lead to a valid state
    let mut valid = true;
    for &value in &sequence {
        if value < average {
            valid = false; // If any value is less than the average, it's invalid
            break;
        }
    }

    // Output the result based on the validity of the sequence
    if valid {
        println!("YES");
    } else {
        println!("NO");
    }
}
