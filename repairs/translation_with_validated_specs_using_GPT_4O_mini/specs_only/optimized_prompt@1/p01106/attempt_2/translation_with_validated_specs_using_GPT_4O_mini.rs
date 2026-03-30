fn process_commands(n: usize, i: i64, j: i64) -> String {
    // Check precondition: n must be greater than 0
    if n == 0 {
        // Graceful termination for non-positive n
        return String::new();
    }

    let mut result = String::with_capacity(n); // Preallocate string capacity for efficiency

    for bit in 0..n {
        // Extract the bit at position `bit` from `i` and `j`
        let i_bit = (i >> bit) & 1; // Get the bit from i
        let j_bit = (j >> bit) & 1; // Get the bit from j

        // Determine the command based on the specification
        if i_bit == 0 {
            // If the bit in i is 0, negate the corresponding bit in j
            if j_bit == 0 {
                result.push('R'); // 0 negated is 1, so 'R'
            } else {
                result.push('L'); // 1 negated is 0, so 'L'
            }
        } else {
            // If the bit in i is 1, use the bit in j directly
            if j_bit == 0 {
                result.push('L'); // 0 in j corresponds to 'L'
            } else {
                result.push('R'); // 1 in j corresponds to 'R'
            }
        }
    }

    result // Return the constructed string
}

fn main() {
    // Example usage of the process_commands function
    let n = 5; // Number of bits
    let i = 10; // Example integer i
    let j = 3;  // Example integer j
    let commands = process_commands(n, i, j);
    println!("{}", commands); // Output the result
}
