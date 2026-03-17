fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the first line containing n and k
    let first_line = reader.lines().next().unwrap().unwrap();
    let mut params = first_line.split_whitespace();
    let n: i64 = params.next().unwrap().parse().unwrap(); // Total number of integers
    let k: i64 = params.next().unwrap().parse().unwrap(); // Maximum size of each group

    // Preconditions: n > 0 and k > 1
    if n <= 0 || k <= 1 {
        // Undefined behavior as per specification; we can panic here to mimic potential crash
        panic!("Invalid input: n must be > 0 and k must be > 1");
    }

    // Read the array of integers
    let mut a: Vec<i64> = Vec::with_capacity(n as usize);
    for line in reader.lines().take(n as usize) {
        let value: i64 = line.unwrap().parse().unwrap();
        a.push(value);
    }

    // The number of groups needed is calculated by dividing n by k and rounding up
    let groups = (n + k - 1) / k; // This emulates ceiling division for positive integers

    // Output the result
    println!("{}", groups);
}
