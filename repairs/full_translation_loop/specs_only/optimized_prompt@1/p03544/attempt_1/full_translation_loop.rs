fn compute_nth_term(n: usize) -> i64 {
    // The first two terms of the sequence
    let mut a = 2; // Term 0
    let mut b = 1; // Term 1

    // Handle the case for n = 0 and n = 1 directly
    if n == 0 {
        return a; // Return the first term
    } else if n == 1 {
        return b; // Return the second term
    }

    // Compute the nth term iteratively for n >= 2
    let mut nth_term = 0;
    for _ in 2..=n {
        nth_term = a + b; // Compute the next term
        a = b; // Update a to the previous term
        b = nth_term; // Update b to the current term
    }

    nth_term // Return the nth term
}
