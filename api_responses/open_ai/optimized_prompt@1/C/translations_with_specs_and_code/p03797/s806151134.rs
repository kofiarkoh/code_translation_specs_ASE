use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let nums: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();

    let n = nums[0];
    let m = nums[1];

    // Preconditions check
    if m < 1 {
        panic!("Precondition violated: m must be greater than or equal to 1");
    }

    let sum: i64 = if m == 1 {
        0
    } else if (m / 2 - n) < 0 {
        m / 2
    } else if (m / 2 - n) < 2 {
        n
    } else {
        n + (m / 2 - n) / 2
    };

    // Postcondition: Ensure the output is non-negative
    assert!(sum >= 0, "Postcondition violated: output must be non-negative");

    // Output the result
    println!("{}", sum);
}
