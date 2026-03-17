fn main() {
    use std::io::{self, BufRead};

    // Read input from stdin
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line which contains the number of subsequent integers
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input");

    // Ensure that the number of integers is positive as per the specification
    assert!(n > 0, "The first input must be a positive integer.");

    let mut ans = 0;

    // Read the subsequent integers
    for _ in 0..n {
        let value: i32 = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input");
        ans += value - 1; // Decrement each value by one and accumulate the sum
    }

    // Output the result
    println!("{}", ans);
}
