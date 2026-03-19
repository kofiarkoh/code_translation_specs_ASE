use std::io::{self, BufRead};

fn main() {
    // Create a buffer to read input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    // Initialize counts for 'o' and 'x'
    let mut count_o = 0;
    let mut count_x = 0;

    // Count occurrences of 'o' and 'x'
    for ch in input.chars() {
        match ch {
            'o' => count_o += 1,
            'x' => count_x += 1,
            _ => {}
        }
    }

    // Output 'o' if it appears more frequently, otherwise output 'x'
    // Preference for 'x' in case of a tie
    if count_o < count_x {
        println!("o");
    } else {
        println!("x");
    }
}
