fn main() {
    // Example input values
    let n: usize = 5; // Number of elements
    let x: [i32; 110] = [1, 2, 3, 4, 5]; // Initial values
    let a: [[i32; 110]; 3] = [[1, 2, 3, 4, 5], [5, 4, 3, 2, 1], [0, 0, 0, 0, 0]]; // 2D array

    // Call the function to process the input
    let result = find_min_adjustment(n, &x, &a);
    
    // Output the result
    match result {
        Some(value) => println!("{}", value),
        None => println!("Saiko"),
    }
}

// Function to find the minimum adjustment
fn find_min_adjustment(n: usize, x: &[i32; 110], a: &[[i32; 110]; 3]) -> Option<i32> {
    let mut min_adjustment: Option<i32> = None;

    // Iterate over each element in x
    for i in 0..n {
        // Calculate the modified value
        let modified_value = a[0][i] + a[1][i] + a[2][i];

        // Calculate the adjustment needed
        let adjustment = modified_value - x[i];

        // Count how many elements exceed the threshold
        let count_exceeding = (0..n).filter(|&j| x[j] + adjustment > 10).count();

        // Check if the count exceeds the limit
        if count_exceeding <= 2 {
            // Update the minimum adjustment if necessary
            if min_adjustment.is_none() || adjustment < min_adjustment.unwrap() {
                min_adjustment = Some(adjustment);
            }
        }
    }

    // Return the minimum adjustment found, or None if no valid adjustment exists
    min_adjustment
}
