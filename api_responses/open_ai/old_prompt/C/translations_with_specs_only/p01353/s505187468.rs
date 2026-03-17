fn calculate_score(n: usize, h: &[i32], a: &[i32], d: &[i32], s: &[i32]) -> i64 {
    // Check for the edge case where there are no players
    if n == 0 {
        return -1; // Output -1 if there are no players
    }

    // Ensure all input arrays have the correct length
    if h.len() != n || a.len() != n || d.len() != n || s.len() != n {
        return -1; // Output -1 if input arrays do not match the number of players
    }

    // Initialize score
    let mut score: i64 = 0;

    // Iterate through each player to calculate the score
    for i in 0..n {
        // Check for non-negative attributes as per the specification
        if h[i] < 0 || a[i] < 0 || d[i] < 0 || s[i] < 0 {
            return -1; // Output -1 if any attribute is negative
        }

        // Calculate score based on player attributes
        // The score calculation is assumed to be a simple combination of attributes
        score += (h[i] + a[i] - d[i]) as i64 + (s[i] * 2) as i64; // Example calculation
    }

    // Return the calculated score
    score
}

fn main() {
    // Example usage
    let n = 3;
    let h = [10, 20, 30];
    let a = [5, 15, 25];
    let d = [2, 3, 4];
    let s = [1, 2, 3];

    let result = calculate_score(n, &h, &a, &d, &s);
    println!("{}", result); // Output the result
}
