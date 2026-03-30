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
                result.push('L'); // 0 negated is 1, so 'L'
            } else {
                result.push('R'); // 1 negated is 0, so 'R'
            }
        } else {
            // If the bit in i is 1, use the bit in j directly
            if j_bit == 0 {
                result.push('R'); // 0 in j corresponds to 'R'
            } else {
                result.push('L'); // 1 in j corresponds to 'L'
            }
        }
    }

    result // Return the constructed string
}

fn main() {
    // Example usage of the process_commands function
    let inputs = vec![
        (3, 5, 7),
        (4, 10, 12),
        (5, 17, 20),
        (0, 0, 0),
    ];

    for (n, i, j) in inputs {
        let commands = process_commands(n, i, j);
        println!("{}", commands); // Output the result
    }
}
