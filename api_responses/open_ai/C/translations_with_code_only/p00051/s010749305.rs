// translated from the provided C program
// Preserves the exact semantics as given by the C code (use of 32-bit signed ints,
// truncating division/remainder semantics, explicit selection-sort, digit composition).
use std::io::{self, Read};

fn main() {
    // Read all stdin and split by whitespace, similar to successive scanf("%d", ...)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();

    // Parse n as i32 to match C's `int n; scanf("%d",&n);`
    let n: i32 = match toks.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    // If n <= 0, the C for-loop `for(i=0;i<n;i++)` does zero iterations.
    if n <= 0 {
        return;
    }

    for _ in 0..(n as usize) {
        // Read m for this test case
        let mut m: i32 = match toks.next() {
            Some(s) => s.parse().unwrap(),
            None => return, // mimic stdin exhaustion by exiting
        };

        // num[8]
        let mut num = [0i32; 8];

        // for(j=0;j<8;j++){
        //   num[j]=m%10;
        //   m/=10;
        // }
        for j in 0..8 {
            // Rust's % and / on signed integers have the same trunc-toward-zero semantics
            // as the C code expects.
            num[j] = m % 10;
            m /= 10;
        }

        // selection-like sort from the C code:
        for j in 0..8 {
            for k in j..8 {
                if num[j] > num[k] {
                    let t = num[j];
                    num[j] = num[k];
                    num[k] = t;
                }
            }
        }

        // Build `min` using the same stepwise pattern as the C code
        let mut min: i32 = 0;
        for j in 0..8 {
            min += num[j];
            if j < 7 {
                min *= 10;
            }
        }

        // Build `max` by iterating j = 7..=0 descending, matching the C loop.
        let mut max: i32 = 0;
        let mut j: i32 = 7;
        while j >= 0 {
            max += num[j as usize];
            if j > 0 {
                max *= 10;
            }
            j -= 1;
        }

        // Print result exactly as `printf("%d\n", max-min);`
        println!("{}", max - min);
    }
}
