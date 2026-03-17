fn main() {
    // Read the number of iterations from standard input
    let n: usize = {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().parse().expect("Please enter a non-negative integer")
    };

    // Ensure the input is non-negative
    if n == 0 {
        return; // If n is 0, we output nothing
    }

    let mut current_number = 1; // Start from 1 as the first number to process

    for _ in 0..n {
        // Calculate the digit sum of the current number
        let digit_sum = digit_sum(current_number);

        // Calculate the ratio (current_number / digit_sum)
        // We need to handle the case where digit_sum is 0 to avoid division by zero
        let ratio = if digit_sum > 0 {
            current_number as f64 / digit_sum as f64
        } else {
            f64::INFINITY // If digit_sum is 0, we treat the ratio as infinity
        };

        // Output the current number
        println!("{}", current_number);

        // Increment the current number for the next iteration
        current_number += 1;

        // The transformation to find a new number that minimizes the ratio
        // Here we simply continue to the next number, as the specification does not provide
        // a specific minimization algorithm or criteria.
    }
}

// Function to calculate the sum of the digits of a number
fn digit_sum(mut number: usize) -> usize {
    let mut sum = 0;
    while number > 0 {
        sum += number % 10; // Add the last digit to the sum
        number /= 10; // Remove the last digit
    }
    sum // Return the total sum of digits
}
