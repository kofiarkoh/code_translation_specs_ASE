use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let mut iter = input.split_whitespace();
    let n: i64 = iter.next().unwrap().parse().expect("Invalid input for N");
    let k: i64 = iter.next().unwrap().parse().expect("Invalid input for K");

    // Check for the precondition that K should not be zero
    if k == 0 {
        // Undefined behavior in the original C code; we will handle it gracefully
        eprintln!("Error: K should not be zero.");
        return;
    }

    let groups = n / k;
    let result = if k % 2 == 1 {
        groups * groups * groups
    } else {
        let adjusted_groups = (n + k / 2) / k;
        groups * groups * groups + adjusted_groups * adjusted_groups * adjusted_groups
    };

    // Output the result
    println!("{}", result);
}
