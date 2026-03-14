use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut results = Vec::new();

    for line in stdin.lock().lines() {
        let n = line.expect("Failed to read line");
        if n.len() > 10 {
            // According to the spec, we assume undefined behavior for invalid input.
            continue;
        }

        let mut seq: Vec<u32> = n.chars()
            .map(|c| c.to_digit(10).expect("Invalid character")) // Assumes input is valid as per spec
            .collect();

        // Reduction process
        for j in 0..9 {
            for k in 0..9 - j {
                seq[k] = (seq[k] + seq[k + 1]) % 10;
            }
        }

        results.push(seq[0]);
    }

    for result in results {
        println!("{}", result);
    }
}
