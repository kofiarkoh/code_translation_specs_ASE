use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        match line {
            Ok(input) => {
                // Attempt to parse the input as a positive integer
                if let Ok(n) = input.trim().parse::<i32>() {
                    if n > 0 {
                        let mut sum: i64 = 0; // Use i64 to avoid overflow
                        // Calculate the sum of squares of multiples of n
                        for i in (n..=600).step_by(n as usize) {
                            sum += (i * i) as i64; // Accumulate the sum of squares
                        }
                        sum *= n as i64; // Multiply by n
                        println!("{}", sum); // Output the result
                    }
                }
                // If parsing fails or n is not positive, we ignore the input as per the spec
            }
            Err(_) => {
                // Handle any potential errors in reading lines, but continue processing
                continue;
            }
        }
    }
}
