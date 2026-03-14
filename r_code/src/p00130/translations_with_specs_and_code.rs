use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let mut lines = input.lines();
    let n: usize = match lines.next() {
        Some(line) => line.parse().expect("Invalid number of sequences"),
        None => return,
    };

    for _ in 0..n {
        let mut l = ['\0'; 27]; // Initialize the constructed string with null characters
        let mut p = 13; // Starting position
        let mut sequence = lines.next().unwrap_or("");

        for c in sequence.chars() {
            if c == '-' || c == '<' {
                // Control characters modify the position
                if c == '-' {
                    p += 1; // Move right
                } else {
                    p -= 1; // Move left
                }
            } else {
                // Break on non-control character
                l[p] = c; // Insert character at the current position
                break;
            }
        }

        // Find the first non-zero character
        let start_index = l.iter().position(|&x| x != '\0').unwrap_or(0);
        let output: String = l[start_index..].iter().collect();
        println!("{}", output);
    }
}
