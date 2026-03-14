use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap(); // Unwrap the Result to get the String
        let mut a: [f64; 8] = [0.0; 8]; // Initialize an array of 8 f64

        // Split the line into parts and parse them into the array
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 8 {
            break; // If we don't have exactly 8 parts, we stop reading
        }

        for (i, part) in parts.iter().enumerate() {
            a[i] = part.parse().unwrap(); // Parse each part into f64
        }

        // Check the condition as per the C code
        if (a[1] - a[3]) * (a[5] - a[7]) + (a[0] - a[2]) * (a[4] - a[6]) == 0.0 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
