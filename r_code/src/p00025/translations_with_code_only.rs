use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated tokens, parse as i32.
    // This mirrors the C program which reads integers with scanf("%d").
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse tokens into integers. If parsing fails, let it panic (like scanf would on bad input).
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // Process in groups of 8 integers: first 4 -> A, next 4 -> B.
    // The original C code loops while it can read 4 ints for A, then reads 4 for B.
    // Here we only process complete 8-int records (typical for well-formed input).
    for chunk in nums.chunks(8) {
        if chunk.len() < 8 {
            // If there are not enough ints for a full record, stop.
            // The original C code calls scanf for B without checking its return value,
            // which would lead to undefined/implementation-defined behavior on truncated input.
            // To remain robust while keeping intended behavior for valid inputs, we ignore incomplete trailing data.
            break;
        }

        let a = &chunk[0..4];
        let b = &chunk[4..8];

        let mut hit = 0;
        let mut blow = 0;

        for ai in 0..4 {
            for bi in 0..4 {
                if a[ai] == b[bi] {
                    if ai == bi {
                        hit += 1;
                    } else {
                        blow += 1;
                    }
                }
            }
        }

        println!("{} {}", hit, blow);
    }
}
