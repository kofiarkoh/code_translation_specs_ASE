fn calculate_result(n: usize, pairs: &[(i64, i64)]) -> i64 {
    // Step 1: Calculate the initial cumulative value
    let mut cumulative_value: i64 = 0;

    // The formula is (N - i + 2) * (N - i + 1) / 2 for i from 1 to N
    for i in 1..=n {
        // Cast to i64 to avoid type mismatch
        cumulative_value += ((n - i + 2) as i64) * ((n - i + 1) as i64) / 2;
    }

    // Step 2: Adjust the cumulative value based on the pairs
    for (u, w) in pairs {
        // Ensure the first integer of each pair is less than or equal to the second
        if *u <= *w {
            // Cast n to i64 to avoid type mismatch
            cumulative_value -= *u * (n as i64 - *w + 1);
        }
    }

    // Step 3: Ensure the cumulative value is non-negative
    if cumulative_value < 0 {
        cumulative_value = 0;
    }

    cumulative_value
}

// Main function to demonstrate usage
fn main() {
    let result = calculate_result(5, &[(1, 3), (2, 4)]);
    println!("{}", result);
}
