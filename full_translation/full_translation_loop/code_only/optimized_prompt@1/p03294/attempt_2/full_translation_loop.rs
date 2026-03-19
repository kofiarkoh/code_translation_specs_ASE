use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Read the first line to get N
    let mut lines = handle.lines();
    let N: usize = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse N");

    // Create a vector to hold the integers
    let mut ans = 0;

    // Read N integers
    for _ in 0..N {
        let value: i32 = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse integer");
        ans += value - 1; // Preserve the same logic as in C
    }

    // Print the result
    println!("{}", ans);
}
