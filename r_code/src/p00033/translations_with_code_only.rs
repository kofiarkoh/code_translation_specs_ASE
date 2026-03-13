// main.rs
// Translated from the provided C program.
// Semantic notes:
// - Uses i32 to mirror C `int` behavior for comparisons and assignments.
// - Reading is performed by consuming all whitespace-separated tokens from stdin.
//   This matches the C program's use of scanf("%d", ...) for whitespace-separated ints.
// - The loop over test cases is implemented as a signed loop (while i < n) to preserve
//   the C behavior when `n` is non-positive (the loop is not entered).
// - Parsing failures will cause the program to panic (similar to undefined/erroneous input
//   conditions in the original C program using scanf).

use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated tokens (like successive scanf calls).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();

    // Read n (number of test cases). If missing, exit (C would have undefined behavior;
    // here we simply return).
    let n = match toks.next() {
        Some(x) => x.parse::<i32>().unwrap(),
        None => return,
    };

    // Mirror C: for (i = 0; i < n; i++)
    let mut i: i32 = 0;
    while i < n {
        // Read 10 integers into array a.
        let mut a = [0i32; 10];
        for j in 0..9 {
            a[j] = toks.next().unwrap().parse::<i32>().unwrap();
        }
        a[9] = toks.next().unwrap().parse::<i32>().unwrap();

        // Apply the same logic as the C program.
        let mut b = a[0];
        let mut c = 0i32;
        let mut nflag = false;

        for j in 1..10 {
            let v = a[j];
            if b < v {
                b = v;
            } else if c < v {
                c = v;
            } else {
                println!("NO");
                nflag = true;
                break;
            }
        }

        if !nflag {
            println!("YES");
        }

        i += 1;
    }
}
