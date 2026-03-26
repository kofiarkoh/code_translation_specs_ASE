use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line for n
    let n: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    // Read the second line for k
    let k: i32 = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..k {
        // Read ai and bi
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace();
        let ai: i32 = nums.next().unwrap().parse().unwrap();
        let bi: i32 = nums.next().unwrap().parse().unwrap();

        // Adjust ai
        let ai = if n + 1 - ai < ai {
            n + 1 - ai
        } else {
            ai
        };

        // Adjust bi
        let bi = if n + 1 - bi < bi {
            n + 1 - bi
        } else {
            bi
        };

        // Calculate c
        let c = if ai > bi {
            (bi - 1) % 3 + 1
        } else {
            (ai - 1) % 3 + 1
        };

        // Print the result
        println!("{}", c);
    }
}
