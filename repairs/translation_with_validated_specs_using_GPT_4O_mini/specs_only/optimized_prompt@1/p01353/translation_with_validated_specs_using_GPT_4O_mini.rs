fn calculate_score(n: usize, h: &[u32], a: &[u32], d: &[u32], s: &[u32]) -> Result<u64, &'static str> {
    // Check preconditions
    if n == 0 || n > 40000 {
        return Err("-1");
    }
    
    // Check if all input values are non-negative
    for &value in h.iter().chain(a.iter()).chain(d.iter()).chain(s.iter()) {
        if value < 0 {
            return Err("-1");
        }
    }

    // Check the attack value of the first entity against the defense values of others
    for i in 1..=n {
        if a[0] <= d[i] {
            return Err("-1");
        }
    }

    let mut score: u64 = 0;

    // Iterate through each entity
    for i in 1..=n {
        // Update score only if attack exceeds the first entity's defense
        if a[i] > d[0] {
            let turns = (a[i] - d[0]) as u64; // Calculate turns
            score += turns * s[i] as u64; // Update score based on turns and entity score
        }
    }

    // Check if the health of the first entity is less than or equal to the computed score
    if h[0] <= score as u32 {
        return Err("-1");
    }

    Ok(score) // Return the computed score
}

fn main() {
    // Example usage of the calculate_score function
    let n = 3;
    let h = [100, 30, 10, 5];
    let a = [80, 20, 5, 4];
    let d = [85, 22, 7, 3];
    let s = [0, 1, 2, 3];

    match calculate_score(n, &h, &a, &d, &s) {
        Ok(score) => println!("{}", score),
        Err(err) => println!("{}", err),
    }
}
