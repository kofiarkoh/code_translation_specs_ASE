fn process_pairs(pairs: Vec<(i64, i32)>) -> i64 {
    // Initialize cumulative sum of regular integers and minimum long integer
    let mut cumulative_sum = 0i32;
    let mut min_long = i64::MAX;
    let mut max_value = i64::MIN;

    // Iterate through each pair
    for (long_int, reg_int) in pairs {
        // Update cumulative sum of regular integers
        cumulative_sum += reg_int;

        // Update minimum long integer encountered
        if long_int < min_long {
            min_long = long_int;
        }

        // Calculate the current value based on cumulative sum and minimum long integer
        let current_value = cumulative_sum as i64 - min_long;

        // Update the maximum value if the current value is greater
        if current_value > max_value {
            max_value = current_value;
        }
    }

    // Return the maximum computed value
    max_value
}

fn main() {
    // Example usage
    let pairs = vec![(10, 1), (5, 2), (15, 3)];
    let result = process_pairs(pairs);
    println!("{}", result); // Output will depend on the input pairs
}
