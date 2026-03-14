// Aizu Vol0 0031: Weight
// 2017.7.29

use std::io::{self, BufRead};

const W: [i32; 10] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512];

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let buf = line.expect("Failed to read line");
        let x: i32 = buf.trim().parse().unwrap_or(0); // Handle invalid input by treating it as 0

        let mut f = true; // Flag to control spacing
        let mut x = x; // Mutable copy of x for bit manipulation

        for (i, &weight) in W.iter().enumerate() {
            if x > 0 {
                if x & 1 != 0 {
                    if f {
                        f = false; // First output, no space
                    } else {
                        print!(" "); // Print space before subsequent outputs
                    }
                    print!("{}", weight); // Print the weight
                }
                x >>= 1; // Right shift x to process the next bit
            }
        }
        println!(); // Newline after processing each input
    }
}
