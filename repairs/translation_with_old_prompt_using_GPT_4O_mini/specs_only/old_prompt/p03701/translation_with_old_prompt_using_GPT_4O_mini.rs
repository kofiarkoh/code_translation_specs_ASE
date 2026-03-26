use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of integers
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(0),
        _ => return, // Exit if input is not valid
    };

    // Initialize variables
    let mut sum = 0;
    let mut min_non_multiple_of_ten = 101; // Start with 101, as per specification
    let mut has_non_multiple_of_ten = false; // Track if there's any non-multiple of ten

    // Read integers and compute sum and minimum non-multiple of ten
    for _ in 0..n {
        let value: i32 = match lines.next() {
            Some(Ok(line)) => line.trim().parse().unwrap_or(0), // Default to 0 on parse error
            _ => return, // Exit if input is not valid
        };
        sum += value;

        // Check if the value is a non-multiple of ten
        if value % 10 != 0 {
            has_non_multiple_of_ten = true; // Found a non-multiple of ten
            if value < min_non_multiple_of_ten {
                min_non_multiple_of_ten = value;
            }
        }
    }

    // Check if the sum is a multiple of ten
    if sum % 10 == 0 {
        // If the sum is a multiple of ten and there are no non-multiples of ten
        if !has_non_multiple_of_ten {
            println!("0"); // Output 0 if all are multiples of ten
        } else {
            println!("{}", sum); // Output the sum as is
        }
    } else {
        println!("{}", sum); // Output the sum as is
    }
}
