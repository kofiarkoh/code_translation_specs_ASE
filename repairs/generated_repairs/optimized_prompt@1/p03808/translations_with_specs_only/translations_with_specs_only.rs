fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line for the number of elements
    let n: usize = match lines.next() {
        Some(Ok(line)) => match line.parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid number format");
                return;
            }
        },
        _ => {
            eprintln!("Failed to read the number of elements");
            return;
        }
    };

    // Check precondition: number of integers must not exceed 100,000
    if n < 1 || n > 100_000 {
        eprintln!("Number of elements out of bounds");
        return;
    }

    // Read the sequence of integers
    let mut sequence = Vec::with_capacity(n);
    for line in lines.take(n) {
        match line {
            Ok(num_str) => match num_str.parse::<i32>() {
                Ok(num) => sequence.push(num),
                Err(_) => {
                    eprintln!("Invalid integer format");
                    return;
                }
            },
            Err(_) => {
                eprintln!("Failed to read line");
                return;
            }
        }
    }

    // Evaluate the condition on the sequence
    let result = evaluate_sequence(&sequence);

    // Output the result
    println!("{}", result);
}

// Function to evaluate the sequence based on the specified condition
fn evaluate_sequence(sequence: &[i32]) -> &'static str {
    // Placeholder for the actual condition logic
    // The specification does not define the condition, so we will assume a simple one
    // For example, we can check if the sum of the sequence is even
    let sum: i32 = sequence.iter().sum();

    // Return "YES" if the sum is even, otherwise "NO"
    if sum % 2 == 0 {
        "YES"
    } else {
        "NO"
    }
}
