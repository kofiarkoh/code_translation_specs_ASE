use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line for n and k
    let first_line = lines.next().unwrap().expect("Failed to read the first line");
    let mut parts = first_line.split_whitespace();
    let n: i32 = parts.next().expect("Failed to parse n").parse().expect("n is not a valid integer");
    let k: i32 = parts.next().expect("Failed to parse k").parse().expect("k is not a valid integer");

    // Process each query
    for _ in 0..k {
        let query_line = lines.next().unwrap().expect("Failed to read query line");
        let mut query_parts = query_line.split_whitespace();
        let mut ai: i32 = query_parts.next().expect("Failed to parse ai").parse().expect("ai is not a valid integer");
        let mut bi: i32 = query_parts.next().expect("Failed to parse bi").parse().expect("bi is not a valid integer");

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
