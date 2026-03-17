// Function to count non-overlapping occurrences of the substring "ABC" in a given string.
fn count_abc_occurrences(n: usize, input: &str) -> usize {
    // Preconditions check: if n is less than 3, return 0 as no occurrences can exist.
    if n < 3 {
        return 0;
    }

    // Ensure the input string is null-terminated and has at least n characters.
    // In Rust, we assume the input is valid as per the specification.
    let input_length = input.len();
    if input_length < n {
        // Behavior is not defined if the string is shorter than specified length.
        // Returning 0 to handle this case gracefully.
        return 0;
    }

    let mut count = 0; // Initialize count of occurrences.
    let mut i = 0; // Index to iterate through the string.

    // Iterate through the string without exceeding its bounds.
    while i <= input_length - 3 { // Ensure we can check for "ABC"
        // Check for the substring "ABC".
        if &input[i..i + 3] == "ABC" {
            count += 1; // Increment count if found.
            i += 3; // Move index forward by 3 to ensure non-overlapping.
        } else {
            i += 1; // Move to the next character.
        }
    }

    count // Return the final count of occurrences.
}

fn main() {
    let result = count_abc_occurrences(5, "ABCABCXYZ");
    println!("Count of 'ABC': {}", result); // Expected output: 2
}
