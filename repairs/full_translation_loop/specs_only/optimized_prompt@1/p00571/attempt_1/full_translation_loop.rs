use std::cmp;

fn process_pairs(pairs: Vec<(i64, i32)>) -> i64 {
    // Initialize the maximum computed value and cumulative sum
    let mut max_value = i64::MIN; // Start with the smallest possible long long
    let mut cumulative_sum = 0; // Cumulative sum of the second elements

    // Iterate through the sorted pairs
    for (first, second) in pairs {
        cumulative_sum += second; // Update cumulative sum
        let computed_value = first + cumulative_sum as i64; // Compute the value based on the pair
        max_value = cmp::max(max_value, computed_value); // Update max_value if computed_value is greater
    }

    max_value // Return the maximum computed value
}

fn main() {
    // Example usage (input handling would be added here in a complete program)
    let n = 3; // Number of pairs
    let pairs = vec![(1, 2), (2, 3), (3, 4)]; // Example pairs, should be sorted by the first element

    // Call the processing function and print the result
    let result = process_pairs(pairs);
    println!("{}", result); // Output the maximum computed value
}
