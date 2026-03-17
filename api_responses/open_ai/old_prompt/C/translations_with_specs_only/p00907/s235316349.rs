fn main() {
    // Read input values
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input into parts
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    // Parse the first part as the number of values (d)
    let d: usize = parts[0].parse().expect("Invalid input for d");
    
    // Ensure d is within the valid range
    assert!(d > 0 && d <= 7, "d must be between 1 and 7");
    
    // Parse the remaining parts as the array of doubles (v)
    let v: Vec<f64> = parts[1..=d + 2].iter()
        .map(|&s| s.parse().expect("Invalid input for v"))
        .collect();
    
    // Ensure the size of v is exactly d + 3
    assert_eq!(v.len(), d + 3, "The size of v must be d + 3");
    
    // Initialize the index of the outlier
    let mut outlier_index = d + 2; // Default to the last index if no outlier is found

    // Calculate expected values and identify the outlier
    for i in 0..=d + 2 {
        let mut expected_value = 0.0;
        let mut count = 0;

        // Calculate expected value based on other values
        for j in 0..=d + 2 {
            if i != j {
                expected_value += v[j];
                count += 1;
            }
        }

        // Average the expected value
        expected_value /= count as f64;

        // Check if the current value is significantly different from the expected value
        if (v[i] - expected_value).abs() > 1e-5 {
            outlier_index = i; // Update the outlier index
            break; // Exit the loop after finding the first outlier
        }
    }

    // Print the index of the outlier
    println!("{}", outlier_index);
}
