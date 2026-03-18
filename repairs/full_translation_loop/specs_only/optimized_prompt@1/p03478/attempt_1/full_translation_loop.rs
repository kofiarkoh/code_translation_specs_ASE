fn sum_of_valid_integers(n: usize, a: i32, b: i32) -> i32 {
    // Initialize the cumulative sum
    let mut count = 0;

    // Iterate through each integer from 1 to N
    for i in 1..=n {
        // Calculate the sum of the digits of i
        let digit_sum: i32 = i.to_string().chars()
            .filter_map(|c| c.to_digit(10)) // Convert each character to a digit
            .map(|d| d as i32) // Convert from u32 to i32
            .sum(); // Sum the digits

        // Check if the digit sum is within the range [A, B]
        if digit_sum >= a && digit_sum <= b {
            count += i as i32; // Add i to the cumulative sum
        }
    }

    count // Return the total sum of valid integers
}

// Example usage (uncomment to test):
// fn main() {
//     let result = sum_of_valid_integers(10, 1, 5);
//     println!("{}", result); // Should print the sum of valid integers
// }
