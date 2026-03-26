fn calculate_score(n: usize, h: &[i32], a: &[i32], d: &[i32], s: &[i32]) -> i64 {
    // Check for the edge case where there are no players
    if n == 0 {
        return -1; // Output -1 if there are no players
    }

    // Ensure all attributes are non-negative
    for &health in h.iter() {
        if health < 0 {
            return -1; // Invalid health value
        }
    }
    for &attack in a.iter() {
        if attack < 0 {
            return -1; // Invalid attack value
        }
    }
    for &defense in d.iter() {
        if defense < 0 {
            return -1; // Invalid defense value
        }
    }
    for &special in s.iter() {
        if special < 0 {
            return -1; // Invalid special attribute value
        }
    }

    // Calculate the score based on the provided attributes
    let mut score: i64 = 0;
    for i in 0..n {
        // Adjusted scoring logic to ensure valid game state
        if h[i] < d[i] {
            return -1; // Invalid state if health is less than defense
        }
        score += (h[i] as i64) + (a[i] as i64) - (d[i] as i64) + (s[i] as i64);
    }

    // Return the calculated score
    score
}

fn main() {
    // Example usage of the calculate_score function
    let n = 3;
    let h = [100, 80, 90];
    let a = [30, 20, 25];
    let d = [10, 5, 8];
    let s = [5, 4, 6];

    let result = calculate_score(n, &h, &a, &d, &s);
    println!("{}", result); // Output the result
}
