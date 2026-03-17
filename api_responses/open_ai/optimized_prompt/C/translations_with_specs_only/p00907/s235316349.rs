fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line as the number of values
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Ensure we have at least n + 3 floating-point numbers
    let total_values = n + 3;
    let mut values: Vec<f64> = Vec::with_capacity(total_values);
    
    // Read the floating-point numbers
    for _ in 0..total_values {
        let value: f64 = lines.next().unwrap().unwrap().parse().unwrap();
        values.push(value);
    }

    // Identify the outlier index
    let outlier_index = identify_outlier(&values);
    
    // Output the index of the outlier or the last value processed
    println!("{}", outlier_index);
}

// Function to identify the outlier index based on interpolation
fn identify_outlier(values: &[f64]) -> usize {
    // Placeholder for outlier detection logic
    // This is a simple example; the actual logic should be defined based on the specification
    let mut outlier_index = values.len() - 1; // Default to the last index

    // Example logic to find an outlier (this should be replaced with actual criteria)
    for i in 1..values.len() - 1 {
        if (values[i] - values[i - 1]).abs() > (values[i + 1] - values[i]).abs() * 2.0 {
            outlier_index = i; // Found an outlier
            break;
        }
    }

    outlier_index
}
