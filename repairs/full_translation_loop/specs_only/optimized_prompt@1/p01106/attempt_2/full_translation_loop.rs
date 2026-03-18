use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();

        // Read n, i, j from the input
        let n: usize = match parts.next().unwrap().parse() {
            Ok(value) if value > 0 => value,
            _ => break, // Terminate if n is non-positive
        };
        let i: u64 = parts.next().unwrap().parse().unwrap();
        let j: u64 = parts.next().unwrap().parse().unwrap();

        // Generate the output string based on the binary representations of i and j
        let mut output = String::with_capacity(n);
        for bit in 0..n {
            // Check the bit at position `bit` in both i and j
            let command = if (j >> bit) & 1 == 1 {
                'R' // If the bit in j is 1, output 'R'
            } else {
                'L' // If the bit in j is 0, output 'L'
            };
            output.push(command);
        }

        // Print the output string
        println!("{}", output);
    }
}
