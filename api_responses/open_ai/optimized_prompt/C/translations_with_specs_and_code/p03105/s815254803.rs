use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();

    let (a, b, c) = (nums[0], nums[1], nums[2]);

    // Preconditions check: a must be non-zero to avoid division by zero
    if a == 0 {
        // Undefined behavior as per specification; no specific error handling implemented
        return;
    }

    let division_result = b / a; // Integer division

    // Output the smaller of the division result and c
    let result = if division_result <= c { division_result } else { c };
    println!("{}", result);
}
