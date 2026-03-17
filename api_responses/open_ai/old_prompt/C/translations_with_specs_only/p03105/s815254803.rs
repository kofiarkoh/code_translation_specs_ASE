use std::io::{self, Write};

fn main() {
    // Read three integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut nums = input.split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    // Ensure we have exactly three integers
    if nums.len() != 3 {
        // Undefined behavior as per specification; we could panic or exit
        panic!("Expected exactly three integers");
    }

    let a = nums[0];
    let b = nums[1];
    let c = nums[2];

    // Check that 'a' is not zero to avoid division by zero
    if a == 0 {
        // Undefined behavior as per specification; we could panic or exit
        panic!("Division by zero is not allowed");
    }

    // Calculate the integer division of b by a
    let division_result = b / a;

    // Determine the minimum of the division result and c
    let result = if division_result < c { division_result } else { c };

    // Print the result followed by a newline
    println!("{}", result);
}
