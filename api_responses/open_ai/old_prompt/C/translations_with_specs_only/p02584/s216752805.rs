fn modified_absolute_value(x: i64, k: u64, d: u64) -> i64 {
    // Calculate the absolute value of x
    let abs_x = x.abs();

    // Determine how many decrements can be applied
    let decrements = k.min(abs_x / d as i64);

    // Calculate the modified absolute value after applying decrements
    let modified_value = abs_x - decrements * d as i64;

    // Return the modified absolute value
    modified_value
}

fn main() {
    // Example usage: reading inputs from standard input
    let x: i64 = 10; // Example input for x
    let k: u64 = 3;  // Example input for k
    let d: u64 = 4;  // Example input for d

    // Call the function and print the result
    let result = modified_absolute_value(x, k, d);
    println!("{}", result);
}
