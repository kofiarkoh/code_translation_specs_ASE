fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();

        // Parse the inputs
        let n: usize = match parts[0].parse() {
            Ok(value) if value > 0 => value,
            _ => break, // Terminate if n is non-positive
        };
        let i: u64 = parts[1].parse().unwrap();
        let j: u64 = parts[2].parse().unwrap();

        // Generate the output string based on the binary representation of i and j
        let mut output = String::with_capacity(n);
        for k in 0..n {
            // Check the k-th bit of both i and j
            let bit_i = (i >> k) & 1;
            let bit_j = (j >> k) & 1;

            // Determine the command based on the bits
            if bit_i == 1 && bit_j == 1 {
                output.push('L');
            } else if bit_i == 0 && bit_j == 0 {
                output.push('R');
            } else {
                // If one is 1 and the other is 0, we can choose either 'L' or 'R'.
                // The specification does not define this case, so we choose 'L' for consistency.
                output.push('L');
            }
        }

        // Print the output string
        println!("{}", output);
    }
}
