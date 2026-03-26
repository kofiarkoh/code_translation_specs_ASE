use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line for n and k
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: i32 = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();

    // Process each query
    for _ in 0..k {
        let query_line = lines.next().unwrap().unwrap();
        let mut query_parts = query_line.split_whitespace();
        let mut ai: i32 = query_parts.next().unwrap().parse().unwrap();
        let mut bi: i32 = query_parts.next().unwrap().parse().unwrap();

        // Adjust ai and bi if they exceed half of n
        if ai > n / 2 {
            ai = n - ai + 1; // Adjusting to circular range
        }
        if bi > n / 2 {
            bi = n - bi + 1; // Adjusting to circular range
        }

        // Compute the value based on adjusted positions
        let result = (ai + bi) % n; // Assuming the result is derived from the sum modulo n
        println!("{}", result);
    }
}
