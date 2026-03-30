fn compute_matching_value(n: usize, s: &str, t: &str) -> usize {
    // Preconditions: n must be non-negative and s, t must be of length n
    assert!(n >= 0);
    assert_eq!(s.len(), n);
    assert_eq!(t.len(), n);

    let mut last_match_index = None;

    // Iterate from the end of the strings towards the beginning
    for i in (0..n).rev() {
        if s.as_bytes()[i] == t.as_bytes()[i] {
            last_match_index = Some(i);
            break; // Stop at the first match found
        }
    }

    // If a match was found, calculate the result based on the last match index
    match last_match_index {
        Some(i) => 2 * n - (n - i - 1),
        None => 0, // If no matches found, return 0 as per the specification
    }
}
