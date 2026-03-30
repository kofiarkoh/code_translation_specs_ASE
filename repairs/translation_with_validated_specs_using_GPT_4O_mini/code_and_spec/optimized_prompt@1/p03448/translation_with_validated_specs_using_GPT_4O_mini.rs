use std::io;

fn main() {
    let mut input = String::new();
    
    // Read inputs A, B, C, and X
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    
    // Safely parse the inputs, handling potential errors
    let a: usize = iter.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let b: usize = iter.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let c: usize = iter.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let x: usize = iter.next().and_then(|s| s.parse().ok()).unwrap_or(0);

    let mut count = 0;

    // Iterate through all combinations of i, j, and k
    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                // Check if the weighted sum equals X
                let sum = 500 * i + 100 * j + 50 * k;
                if sum == x {
                    count += 1;
                }
                // Early exit if the sum exceeds X
                if sum > x {
                    break; // No need to check further k values
                }
            }
        }
    }

    // Print the result
    println!("{}", count);
}
