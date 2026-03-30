fn count_characters(input: &str) -> char {
    // Initialize counts for 'o' and 'x'
    let mut count_o = 0;
    let mut count_x = 0;

    // Iterate over the input string, counting 'o' and 'x'
    for c in input.chars() {
        match c {
            'o' => count_o += 1,
            'x' => count_x += 1,
            _ => {} // Ignore other characters
        }
    }

    // Determine the output based on the counts
    if count_x > count_o {
        'x'
    } else {
        'o' // This includes the case where counts are equal or both are zero
    }
}

fn main() {
    // Example usage
    let input = "oxxoo"; // Example input
    let result = count_characters(input);
    println!("{}", result); // Output the result
}
