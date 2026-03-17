fn count_abc_occurrences(n: usize, input: &str) -> usize {
    // Ensure that n is within the valid range
    if n < 3 || n > 50 {
        return 0; // If n is less than 3, "ABC" cannot fit, return 0
    }

    let mut count = 0;

    // Iterate through the string, checking for "ABC"
    for i in 0..=n - 3 { // n - 3 to allow for overlapping
        if &input[i..i + 3] == "ABC" {
            count += 1;
        }
    }

    count // Return the count of occurrences
}

fn main() {
    // Example usage
    let n: usize = 5; // This would typically be read from input
    let input: &str = "ABCABC"; // This would also typically be read from input

    // Call the function and print the result
    let result = count_abc_occurrences(n, input);
    println!("{}", result); // Output the count
}
