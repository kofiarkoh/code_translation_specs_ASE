fn count_abc_occurrences(n: usize, input: &str) -> usize {
    // Preconditions check: n must be >= 3
    if n < 3 {
        // Behavior is not defined, but we will return 0 to avoid undefined behavior.
        return 0;
    }

    // Check if the input string is at least n characters long
    if input.len() < n {
        // Behavior is not defined, but we will return 0 to avoid undefined behavior.
        return 0;
    }

    let mut count = 0;
    let mut index = 0;

    // Iterate through the string to count non-overlapping occurrences of "ABC"
    while index <= input.len() - 3 {
        // Check for the substring "ABC"
        if &input[index..index + 3] == "ABC" {
            count += 1;
            // Move index forward by 3 to ensure non-overlapping
            index += 3;
        } else {
            // Move index forward by 1 to continue searching
            index += 1;
        }
    }

    // The count of occurrences is guaranteed to be non-negative
    count
}

fn main() {
    let n = 6; // Example length
    let input = "ABCDEFABC"; // Example input string
    let result = count_abc_occurrences(n, input);
    println!("Count of 'ABC': {}", result);
}
