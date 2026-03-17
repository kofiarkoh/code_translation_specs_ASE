use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line to get `n`
    let n: i32 = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(0),
        _ => 0,
    };

    // If n is zero or negative, output "IMPOSSIBLE" immediately
    if n <= 0 {
        println!("IMPOSSIBLE");
        return;
    }

    let mut p = 0; // Counter for conditions
    let mut possible = true; // Flag to determine if the outcome is possible

    // Read pairs of (a, b) until EOF
    for line in lines {
        if let Ok(line) = line {
            let nums: Vec<i32> = line.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            if nums.len() < 2 {
                continue; // Skip if we don't have enough numbers
            }

            let a = nums[0];
            let b = nums[1];

            // Check the conditions based on the original C logic
            if p != 0 && n > 0 {
                possible = false;
                break;
            }

            if a != 0 && b != n {
                p += 1;
            } else {
                p = 0;
            }
        }
    }

    // Output the result based on the possible flag
    if possible {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}
``` 

### Notes:
1. **Input Handling**: The Rust code reads from standard input and processes each line to extract pairs of integers (a, b).
2. **Immediate Check for `n`**: If `n` is zero or negative, it outputs "IMPOSSIBLE" immediately, adhering to the specification.
3. **Loop Logic**: The loop processes pairs of integers and updates the `p` counter based on the conditions derived from the C code.
4. **Output**: The final output is determined by the `possible` flag, which reflects whether the conditions were met.
5. **Safety**: The code avoids unsafe operations and uses idiomatic Rust practices for error handling and input parsing.
