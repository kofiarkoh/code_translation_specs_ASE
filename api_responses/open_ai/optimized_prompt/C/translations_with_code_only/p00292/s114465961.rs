use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line to get the number of iterations
    let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..n {
        // Read the next line for x and y
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace();
        let x: i32 = nums.next().unwrap().parse().unwrap();
        let y: i32 = nums.next().unwrap().parse().unwrap();

        // Calculate the output as per the original C code
        // (x - 1) % y + 1
        let result = (x - 1) % y + 1;

        // Print the result
        println!("{}", result);
    }
}
