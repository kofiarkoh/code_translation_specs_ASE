// Aizu Vol0 0049: Blood Groups
// 2017.7.29

use std::io::{self, BufRead};

const MAX: usize = 50;

fn main() {
    let mut blood = [0; MAX + 5];
    let mut cnt = [0; 5];

    // Initialize blood and cnt arrays
    for k in 0..=MAX {
        blood[k] = 0;
    }
    for k in 1..5 {
        cnt[k] = 0;
    }

    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let buf = line.expect("Failed to read line");
        let mut p = buf.as_bytes();
        let mut k = 0;

        // Skip leading whitespace
        while !p.is_empty() && p[0].is_ascii_whitespace() {
            p = &p[1..];
        }

        // Read the blood group number
        if !p.is_empty() {
            k = (p[0] - b'0') as usize;
            p = &p[1..];
            if !p.is_empty() && p[0].is_ascii_digit() {
                k = k * 10 + (p[0] - b'0') as usize;
                p = &p[1..];
            }
        }

        // Skip whitespace and the comma
        while !p.is_empty() && p[0].is_ascii_whitespace() {
            p = &p[1..];
        }
        if !p.is_empty() {
            p = &p[1..]; // Skip the comma
        }

        // Skip whitespace again
        while !p.is_empty() && p[0].is_ascii_whitespace() {
            p = &p[1..];
        }

        // Determine blood type
        let t = if p.starts_with(b"AB") {
            3
        } else if p.starts_with(b"B") {
            2
        } else if p.starts_with(b"O") {
            4
        } else {
            1
        };

        // Update blood and cnt arrays
        if blood[k] == 0 {
            blood[k] = t;
            cnt[t] += 1;
        } else if blood[k] != t {
            // Infinite loop to mimic the C code behavior
            loop {}
        }
    }

    // Print the counts
    for k in 1..5 {
        println!("{}", cnt[k]);
    }
}
