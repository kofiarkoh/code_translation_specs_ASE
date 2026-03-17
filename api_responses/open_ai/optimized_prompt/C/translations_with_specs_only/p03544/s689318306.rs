fn compute_nth_term(n: usize) -> i64 {
    // The sequence starts with 2 and 1 for the first two terms.
    let mut sequence = [0i64; 100];
    sequence[0] = 2; // First term
    sequence[1] = 1; // Second term

    // Compute the sequence up to the nth term
    for i in 2..=n {
        sequence[i] = sequence[i - 1] + sequence[i - 2];
    }

    // Return the nth term
    sequence[n]
}

fn main() {
    // Example input; in a real scenario, this would be read from standard input.
    let n: usize = 10; // Replace with actual input handling as needed

    // Ensure n is within the valid range
    if n > 99 {
        // Behavior is not defined for n > 99, so we do nothing or could panic
        return;
    }

    // Compute the nth term
    let result = compute_nth_term(n);

    // Print the result as specified
    println!("{}", result);
}
