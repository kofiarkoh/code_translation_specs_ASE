fn compute_result(n: usize, s: &str, t: &str) -> usize {
    // Preconditions: N must be non-negative and strings S and T must be of length N.
    // If N is negative or strings do not match the specified length, behavior is undefined.
    assert!(n >= 0, "N must be non-negative");
    assert_eq!(s.len(), n, "String S must be of length N");
    assert_eq!(t.len(), n, "String T must be of length N");

    let mut longest_suffix_length = 0;

    // Iterate over possible suffix lengths
    for i in 0..=n {
        // Check if the suffix of S of length i matches the prefix of T of length i
        if s[s.len() - i..] == t[0..i] {
            longest_suffix_length = i;
        }
    }

    // The output integer is guaranteed to be non-negative
    longest_suffix_length
}

fn main() {
    // Example usage
    let n = 5;
    let s = "hello";
    let t = "llohe";
    
    let result = compute_result(n, s, t);
    println!("Computed result: {}", result);
}
