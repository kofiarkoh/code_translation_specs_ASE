fn main() {
    // Read the number of iterations from standard input
    let n: i64 = std::env::args()
        .nth(1)
        .expect("Please provide a non-negative integer as an argument.")
        .parse()
        .expect("Invalid input; please provide a valid long integer.");

    // Ensure n is non-negative
    if n < 0 {
        panic!("n must be a non-negative integer.");
    }

    // Initialize the starting value
    let mut x: i64 = 1; // Starting from 1 as a reasonable initial value

    for _ in 0..n {
        let mut min_ratio = f64::MAX; // Start with the maximum possible ratio
        let mut best_x = x; // Variable to store the best x found

        // Iterate over a fixed range (0 to 15) to find the best x
        for y in 0..15 {
            let current_x = x + y; // Increment x by y
            let sum_of_digits = sum_digits(current_x); // Calculate the sum of digits
            if sum_of_digits > 0 { // Avoid division by zero
                let ratio = current_x as f64 / sum_of_digits as f64; // Calculate the ratio
                if ratio < min_ratio { // Check if we found a new minimum
                    min_ratio = ratio; // Update the minimum ratio
                    best_x = current_x; // Update the best x
                }
            }
        }

        // Update x to the best found value
        x = best_x;

        // Print the current value of x
        println!("{}", x);
    }
}

// Function to calculate the sum of digits of a number
fn sum_digits(mut num: i64) -> i64 {
    let mut sum = 0;
    while num > 0 {
        sum += num % 10; // Add the last digit to the sum
        num /= 10; // Remove the last digit
    }
    sum
}
