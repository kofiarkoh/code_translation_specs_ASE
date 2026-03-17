fn sum_of_valid_integers(n: usize, a: usize, b: usize) -> usize {
    // Initialize the cumulative sum variable
    let mut count = 0;

    // Iterate through each integer from 1 to N
    for i in 1..=n {
        // Calculate the sum of the digits of the current integer
        let digit_sum = sum_of_digits(i);

        // Check if the digit sum is within the specified range [A, B]
        if digit_sum >= a && digit_sum <= b {
            count += i; // Add the integer to the cumulative sum
        }
    }

    count // Return the cumulative sum
}

// Helper function to calculate the sum of the digits of a number
fn sum_of_digits(mut num: usize) -> usize {
    let mut sum = 0;

    // Sum the digits by repeatedly extracting the last digit
    while num > 0 {
        sum += num % 10; // Add the last digit to the sum
        num /= 10; // Remove the last digit
    }

    sum // Return the total sum of the digits
}

fn main() {
    // Example usage
    let n = 20; // Upper limit
    let a = 1;  // Lower bound for digit sum
    let b = 5;  // Upper bound for digit sum

    let result = sum_of_valid_integers(n, a, b);
    println!("The sum of valid integers is: {}", result);
}
