use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the integers from the input
    let mut nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();

    let (a, b, c) = (nums[0], nums[1], nums[2]);
    let (d, e, f);

    // Emulate the C code's conditional logic for sorting
    if a <= b && b <= c {
        d = a; e = b; f = c;
    } else if a <= c && c <= b {
        d = a; e = c; f = b;
    } else if b <= a && a <= c {
        d = b; e = a; f = c;
    } else if b <= c && c <= a {
        d = b; e = c; f = a;
    } else if c <= a && a <= b {
        d = c; e = a; f = b;
    } else { // c <= b && b <= a
        d = c; e = b; f = a;
    }

    // Print the sorted output
    println!("{} {} {}", d, e, f);
}
