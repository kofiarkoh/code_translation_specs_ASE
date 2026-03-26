use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of elements
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.parse().unwrap_or(0),
        _ => return, // Exit if input is invalid
    };

    // Ensure N is positive
    if n == 0 {
        println!("NO");
        return;
    }

    // Initialize the array to hold the integers
    let mut a: Vec<i64> = Vec::with_capacity(n);

    // Read the integers into the array
    for _ in 0..n {
        if let Some(Ok(value)) = lines.next() {
            if let Ok(num) = value.parse::<i64>() {
                a.push(num);
            } else {
                println!("NO");
                return; // Exit if parsing fails
            }
        } else {
            println!("NO");
            return; // Exit if input is invalid
        }
    }

    // Calculate the sum of the elements in the array
    let sum: i64 = a.iter().sum();

    // Calculate the triangular number T_n = n * (n + 1) / 2
    let triangular_number = (n as i64) * (n as i64 + 1) / 2;

    // Check if the sum is divisible by the triangular number
    if sum % triangular_number == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
