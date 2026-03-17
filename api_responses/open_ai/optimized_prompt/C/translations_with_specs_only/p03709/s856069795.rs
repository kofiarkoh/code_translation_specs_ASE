// Define a constant for the maximum number of pairs
const N_MAX: usize = 100; // Assuming a reasonable maximum for demonstration

fn process_pairs(n: usize, pairs: &[(i32, i32)]) -> (String, u32) {
    // Ensure n is within the expected range
    if n > N_MAX {
        panic!("Input exceeds maximum allowed pairs");
    }

    // Initialize a result variable
    let mut result: u32 = 0;
    let mut condition_met = false;

    // Process each pair
    for &(a, b) in pairs.iter().take(n) {
        // Example condition: Check if the sum of the pair is even
        if (a + b) % 2 == 0 {
            condition_met = true;
        }
        // Example computation: Count how many pairs have a positive sum
        if a + b > 0 {
            result += 1; // Increment result for each positive sum
        }
    }

    // Determine output message based on the condition
    let message = if condition_met { "Yes" } else { "No" };

    // Return the output message and the computed result
    (message.to_string(), result)
}

// Example usage (commented out to avoid execution in a library context)
// fn main() {
//     let n = 3;
//     let pairs = [(1, 2), (-1, -2), (3, -1)];
//     let (message, result) = process_pairs(n, &pairs);
//     println!("Output: {}, Result: {}", message, result);
// }
