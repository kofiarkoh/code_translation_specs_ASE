fn check_expression(h: usize, w: usize, k: usize) -> &'static str {
    // Iterate through all combinations of non-negative integers i and j
    for i in 0..=h {
        for j in 0..=w {
            // Calculate the expression (w * i) + (h * j) - (2 * i * j)
            let expression_value = (w * i) + (h * j) - (2 * i * j);
            // Check if the expression equals k
            if expression_value == k {
                return "Yes"; // Found a valid (i, j) pair
            }
        }
    }
    "No" // No valid (i, j) pair found
}

// Example usage
fn main() {
    let h = 3;
    let w = 4;
    let k = 5;
    let result = check_expression(h, w, k);
    println!("{}", result); // Outputs "Yes" or "No"
}
