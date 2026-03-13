// count_digits_sum.rs
//
// Reads pairs of 32-bit integers (n, s) from stdin.
// For each pair except the sentinel (0, 0), prints the number of distinct
// n-element subsets of {0,1,...,9} whose elements sum to s, one per line.
// Stops processing when the sentinel pair (0, 0) is encountered.
//
// Implementation notes:
// - Parses input tokens as i32. If parsing fails or an incomplete pair is
//   encountered, the program stops (spec leaves malformed input behavior unspecified).
// - Counting is done by enumerating all 2^10 subsets (0..1024), matching the
//   specification that the universe is exactly the digits 0..9.
// - Handles n < 0 by producing 0 (no subsets of negative size). For n > 10
//   also produces 0, as required by the specification.
// - The sentinel (0,0) causes immediate termination with no output for that pair.

use std::io::{self, Read};

fn main() {
    // Read all stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // Behavior on I/O error is unspecified by the specification; exit silently.
        return;
    }

    let mut tokens = input.split_whitespace();
    loop {
        // Attempt to get two tokens; if not available, stop processing.
        let t1 = match tokens.next() {
            Some(t) => t,
            None => break,
        };
        let t2 = match tokens.next() {
            Some(t) => t,
            None => break,
        };

        // Parse as i32; if parsing fails, stop (spec leaves malformed input undefined).
        let n: i32 = match t1.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let s: i32 = match t2.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Sentinel: (0,0) => terminate without printing for that pair.
        if n == 0 && s == 0 {
            break;
        }

        // If n is outside [0,10], no valid combinations exist per spec.
        if n < 0 || n > 10 {
            println!("0");
            continue;
        }

        let target_n = n as u32;
        let mut count: u32 = 0;

        // Enumerate all subsets of digits 0..9 using bitmask 0..(1<<10)
        for mask in 0u32..(1u32 << 10) {
            if mask.count_ones() != target_n {
                continue;
            }
            // Compute sum of digits in this subset
            let mut sum: i32 = 0;
            // iterate bits 0..9
            let mut m = mask;
            // faster iteration over set bits
            while m != 0 {
                let bit = m.trailing_zeros(); // 0..9
                sum += bit as i32;
                m &= m - 1; // clear lowest set bit
            }
            if sum == s {
                count += 1;
            }
        }

        println!("{}", count);
    }
}
